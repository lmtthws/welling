use mysql::packets::handshake::{RequestV10, Response41};
use ::std::net::TcpStream;
use {DatabaseClient, ConnectionInfo};

bitflags! {
    pub(crate) struct Capabilities: u32 {
        const CLIENT_LONG_PASSWORD = 1;
        const CLIENT_FOUND_ROWS = 2;
        const CLIENT_LONG_FLAG = 4;
        const CLIENT_CONNECT_WITH_DB = 8;
        const CLIENT_NO_SCHEMA = 16;
        const CLIENT_COMPRESS = 32;
        //const client_odbc = 64; //-unused since version 3.22
        const CLIENT_LOCAL_FILES = 128;
        const CLIENT_IGNORE_SPACE = 256;
        const CLIENT_PROTOCOL_41 = 512;
        const CLIENT_INTERACTIVE = 1024;
        const CLIENT_SSL = 2048;
        const CLIENT_IGNORE_SIGPIPE = 4096;
        const CLIENT_TRANSACTIONS = 8192;
        //const client_reserved = 16384;  //no longer used
        //const client_reserved2 = 32768; //no longer used
        const CLIENT_MULTI_STATEMENTS = 1_u32 << 16;
        const CLIENT_MULTI_RESULTS = 1_u32 << 17;
        const CLIENT_PS_MULTI_RESULTS = 1_u32 << 18;
        const CLIENT_PLUGIN_AUTH = 1_u32 << 19;
        const CLIENT_CONNECT_ATTRS = 1_u32 << 20;
        const CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA = 1_u32 << 21;
        const CLIENT_CAN_HANDLE_EXPIRED_PASSWORDS = 1_u32 << 22;
        const CLIENT_SESSION_TRACK = 1_u32 << 23;
        const CLIENT_DEPRECATE_EOF = 1_u32 << 24;
        const CLIENT_SSL_VERIFY_SERVER_CERT = 1_u32 << 30;
        const CLIENT_OPTIONAL_RESULTSET_METADATA = 1_u32 << 25;
        const CLIENT_REMEMBER_OPTIONS = 1_u32 << 31;
    }
}

pub(crate) fn server_has_capability(server_handshake: RequestV10, capability: Capabilities) -> bool {
    (server_handshake.capabilities & capability.bits()) == capability.bits()
}

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

trait MySqlConnector {
    fn Connect(&mut self) -> Result<RequestV10, String>;
}

impl MySqlConnector for TcpStream {
    fn Connect(&mut self) -> Result<RequestV10, String> {
        RequestV10::read(self)
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
   
    fn Connect(&mut self) -> Result<(),String> {
        if let Ok(mut stream) = TcpStream::connect(&self.server_details.uri) {
            let request: RequestV10 = stream.Connect()?;

            //TODO: initialize the response
            // also, we should try to do as much as we can with Traits and local functions. Trying to store values in structs and then chain methods is not great unless we wrap the whole thing.

            //response.write_to_stream(stream);

            self.connection_stream = Some(stream);
            Ok(())
        } else {
            return Err(String::from("Unable to open TCP connection to designated host"))
        }
    }
}