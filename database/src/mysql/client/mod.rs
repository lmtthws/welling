extern crate ring;

pub mod capabilities;

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
            let pass_hash = ring::digest::digest(&ring::digest::SHA1, b"the password");
            let pass_hash = pass_hash.as_ref();
           
           
            let mut server_bytes: Vec<u8> = vec!(); 
            
            let auth_plugin = request.auth_plugin.unwrap();
            let server_auth_data = auth_plugin.auth_data.as_bytes();
            if server_auth_data.len() != 20 {
                return Err(String::from("Expected 20 bytes of auth data from server for native auth"));
            }
            server_bytes.extend_from_slice(server_auth_data); //verify 20 bytes of auth data
           
           //concat with the SHA1 hash of the SHA1 hash of the password
            let hash_of_pass_hash = &ring::digest::digest(&ring::digest::SHA1, &pass_hash[0..]);
            server_bytes.extend_from_slice(hash_of_pass_hash.as_ref()); //20 server bytes + SHA1 of SHA1 of password

            //take the sha1 of the concatenation
            let server_bytes = ring::digest::digest(&ring::digest::SHA1, &server_bytes[0..]);
            let server_bytes = server_bytes.as_ref(); 

            //XOR with the original SHA password hash
            let mut auth_data: [u8;5] = [0; 5];
            for (ix,(u1,u2)) in pass_hash.into_iter().zip(server_bytes).enumerate() {
                auth_data[ix] = u1 ^ u2;
            }
            

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

