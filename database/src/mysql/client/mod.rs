pub mod capabilities;
pub mod authentication;

use mysql::client::capabilities::Capabilities;
use mysql::client::authentication::{SupportedAuthMethods, AuthenticationResponse};
use mysql::packets::handshake::request::{RequestV10};
use mysql::packets::handshake::response::{Response41};
use mysql::packets::handshake::authentication::{AuthSwitchRequest, AuthSwitchResponse, AuthPlugin};
use mysql::packets::{ReadablePacket, Header, WriteablePacket};
use mysql::packets::protocol_types::*;
use mysql::packets::general_response::{GeneralResponses};
use ::std::net::{TcpStream,Shutdown};
use {DatabaseClient, ConnectionInfo};
use ::std::io::{BufReader,BufWriter};
use ::std::u8;


pub(crate) struct MySqlClient {
    server_details: ConnectionInfo,
    client_options: MySqlClientOptions,
    connection_stream: Option<TcpStream>
}

struct MySqlClientOptions {
    capabilities: capabilities::Capabilities,
    max_packet_size: u32,
    char_set: u8,
}

impl MySqlClientOptions {
    fn default() -> MySqlClientOptions {
        const DEFAULT_MAX_PACKET_SIZE: u32 = 16 * 1024 * 1024;
        const DEFAULT_CHAR_SET: u8 = 1; //TODO - specify correct value

        MySqlClientOptions{
            capabilities: capabilities::client_capabilities(),
            max_packet_size: DEFAULT_MAX_PACKET_SIZE,
            char_set: DEFAULT_CHAR_SET,
        }
    }
}

impl MySqlClient {
    pub(crate) fn new(server_details: ConnectionInfo) -> MySqlClient {
        MySqlClient {
            server_details,
            client_options: MySqlClientOptions::default(),
            connection_stream: None,
        }
    }

    fn build_handshake_response(&self, request: RequestV10) -> Result<Response41,String> {
        let capabilities = self.client_options.capabilities & request.capabilities;
                
        //if guess of auth method by server does not match, when we send our response, we may get a AuthSwitchRequest

        // If the client doesn't plugins, the defaulting is different
        // if client does not support client_secure_connection, as is true here atm, then we'll only use Old Password Authenticaiton
        // if we're doing protocol 4.1 and we support secure_connection, then we'll do Native Authentication
        // but we support plugins, so it's moot.

        let response = self.generate_auth_response(&request.auth_plugin())?;
        let auth_response = LengthEncodedString::from(response.data());
        
        //TODO: handle the case where auth data is between 251 and 255 in length (not a length integer but valid in this case)
        if auth_response.size().value() > (u8::MAX as u64) && !capabilities.contains(Capabilities::CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA) {
            return self.handshake_err(String::from("Authentication data exceeded max allowed value for server"), None)
        }

        let username = NullTerminatedString::from(&self.server_details.username[0..]);

        let init_database = if capabilities.contains(Capabilities::CLIENT_CONNECT_WITH_DB) {
            match self.server_details.init_database {
                None => None,
                Some(ref s) => Some(NullTerminatedString::from(s))
            }
        } else {
            None
        };

        let auth_plugin_name = if capabilities.contains(Capabilities::CLIENT_PLUGIN_AUTH)  {
            Some(NullTerminatedString::from(&*format!("{}", response.method())))
        } else {
            None
        };

        let mut response = Response41{
            header: Header::new_unsized(request.header.sequence_id()),
            capabilities,
            max_packet_size: self.client_options.max_packet_size,
            char_set: self.client_options.char_set,
            username,
            auth_response,
            init_database,
            auth_plugin_name,
            connection_attributes: None,
        };

        response.calculate_header_size()?;

        Ok(response)
    }

    fn generate_auth_response(&self, auth_plugin: &Option<&AuthPlugin>) -> Result<AuthenticationResponse,String> {
        let auth_method: SupportedAuthMethods;
        if let Some(ref plugin) = auth_plugin {
            auth_method = SupportedAuthMethods::from(&*plugin.name);
        } else {
            auth_method = SupportedAuthMethods::default();
        }

        match auth_method.get_auth_response_value(&self.server_details, auth_plugin) {
            Ok(s) => Ok(AuthenticationResponse::new(auth_method, s)),
            Err(e) => return self.handshake_err(String::from("Unable to generate authentication data"), Some(e))
        }
    }

    fn handshake_err<T>(&self, mut new_err: String, base_err: Option<String>) -> Result<T,String> {
        if let Some(ref e) = base_err {
            new_err.push_str(": ");
            new_err.push_str(e);
        }
        return Err(new_err)
    }

    fn continue_authentication(&self, stream: &mut TcpStream) -> Result<AuthSwitchResponse,String>
    {
        let auth_switch_req = AuthSwitchRequest::read(&mut BufReader::new(stream))?;
        let auth_response = self.generate_auth_response(&Some(auth_switch_req.plugin()))?;
        AuthSwitchResponse::new(auth_response.data(), auth_switch_req.header().sequence_id())
    }

    fn wrapup_handshake(&self, stream: &mut TcpStream) -> Result<(),String> {
         match GeneralResponses::read(stream) {
            Ok(GeneralResponses::Okay(_)) => Ok(()),
            Ok(GeneralResponses::Error(e)) => {
                stream.shutdown(Shutdown::Both).unwrap_or(());
                return self.handshake_err(String::from("Server responded with error"), Some(format!("{}: {}", e.error_code(), e.error_message())))
            },
            Err(e) => {
                stream.shutdown(Shutdown::Both).unwrap_or(());
                return self.handshake_err(String::from("Unable to read server response"), Some(e))
            }
        }
    }
}


/*
    pub capabilities: u32,
    pub max_packet_size: u32,
    pub char_set: u8,
    //filler - 23 0s
    pub username: NullTerminatedString, //null-terminated
    pub auth_response: AuthResponse,  //type of auth response determined by CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA capability
    pub init_database: Option<NullTerminatedString>, //if CLIENT_CONNECT_WITH_DB in capabilities - should be in char set from char_set field
    pub auth_plugin_name: Option<NullTerminatedString>, //if CLIENT_PLUGIN_AUTH; this is the method used to generate auth_response value - should be utf8
    pub connection_attributes: Option<ConnectAttributes> //if CLIENT_CONNECT_ATTRS in capabilities
*/




impl DatabaseClient for MySqlClient {
    #[allow(dead_code)]
    fn connect(&mut self) -> Result<(),String> {
        if let Ok(mut stream) = TcpStream::connect(&self.server_details.uri) {
            {
                let request: RequestV10;
                match RequestV10::read(&mut BufReader::new(&mut stream)) {
                    Ok(r) => request = r,
                    Err(e) => {
                        stream.shutdown(Shutdown::Both).unwrap_or(());
                        return self.handshake_err(String::from("Unable to read initial handshake packet"), Some(e))
                    }
                }

                let response: Response41;
                match self.build_handshake_response(request) {
                    Ok(r) => response = r,
                    Err(e) => {
                        stream.shutdown(Shutdown::Both).unwrap_or(());
                        return Err(e)
                    }
                }

                let write_outcome = response.write(&mut BufWriter::new(&mut stream));
                match write_outcome {
                    Ok(_) => (),
                    Err(e) => {
                        stream.shutdown(Shutdown::Both).unwrap_or(());
                        return self.handshake_err(String::from("Failed to send response packet"), Some(format!("{}",e)))
                    }
                }

                let mut buffer = [0;5];
                if let Ok(5) = stream.peek(&mut buffer) {
                    match buffer[4] {
                        0xFE =>{ 
                            let auth_response = self.continue_authentication(&mut stream);
                            match auth_response {
                                Ok(r) => {
                                    let send_result = r.write(&mut BufWriter::new(&mut stream));
                                    match send_result {
                                        Err(e) => {
                                            stream.shutdown(Shutdown::Both).unwrap_or(());
                                            return self.handshake_err(String::from("Authentication failed"), Some(format!("{}",e)))
                                        },
                                        Ok(()) => ()
                                    }
                                },
                                Err(e) => {
                                    stream.shutdown(Shutdown::Both).unwrap_or(());
                                    return self.handshake_err(String::from("Authentication failed"), Some(e))
                                }

                            }
                        },
                        _ => (),
                    }
                }    

                self.wrapup_handshake(&mut stream)?                        
            } 
            self.connection_stream = Some(stream);
            Ok(())
        } else {
            return Err(String::from("Unable to open TCP connection to designated host"))
        }
    }
}
