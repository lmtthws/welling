pub mod capabilities;
pub mod authentication;

use mysql::client::authentication::SupportedAuthMethods;
use mysql::packets::handshake::request::RequestV10;
use mysql::packets::ReadFromBuffer;
use ::std::net::TcpStream;
use {DatabaseClient, ConnectionInfo};
use ::std::io::BufReader;



pub(crate) struct MySqlClient {
    server_details: ConnectionInfo,
    connection_stream: Option<TcpStream>
}

impl MySqlClient {
    pub(crate) fn new(server_details: ConnectionInfo) -> MySqlClient {
        MySqlClient {
            server_details,
            connection_stream: None,
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
    fn connect(&mut self) -> Result<(),String> {
        if let Ok(mut stream) = TcpStream::connect(&self.server_details.uri) {
            {
            let mut reader = BufReader::new(&mut stream);
            
            let request: RequestV10 = RequestV10::read(&mut reader)?;

            //if guess of auth method by server does not match, when we send our response, we may get a AuthSwitchRequest

            // If the client doesn't plugins, the defaulting is different
            // if client does not support client_secure_connection, as is true here atm, then we'll only use Old Password Authenticaiton
            // if we're doing protocol 4.1 and we support secure_connection, then we'll do Native Authentication
            // but we support plugins, so it's moot.

            //Native method - move this to authentication module
            let auth_response = request.auth_plugin.unwrap();
            let auth_response = SupportedAuthMethods::from(&auth_response.name[0..]).get_auth_response_value(&self.server_details, &auth_response);
            

            //TODO: initialize the response
            // also, we should try to do as much as we can with Traits and local functions. Trying to store values in structs and then chain methods is not great unless we wrap the whole thing.

            //response.write_to_stream(stream);

            //we should get an OK packet from the server at the end of this if auth was successful
            }
            self.connection_stream = Some(stream);
            Ok(())
        } else {
            return Err(String::from("Unable to open TCP connection to designated host"))
        }


    }
}

