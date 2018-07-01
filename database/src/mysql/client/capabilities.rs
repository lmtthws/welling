use mysql::packets::handshake::request::RequestV10;

bitflags! {
    pub struct Capabilities: u32 {
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



pub fn client_capabilities() -> Capabilities {
    Capabilities::CLIENT_PROTOCOL_41
    & Capabilities::CLIENT_CONNECT_WITH_DB  //specify default DB in handshake response
    & Capabilities::CLIENT_PLUGIN_AUTH //supports plugin based auth, though we only do native auth atm
    & Capabilities::CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA //see handshake response 4.1
   
    & Capabilities::CLIENT_IGNORE_SPACE // allows for whitespace before '('
    & Capabilities::CLIENT_DEPRECATE_EOF //expect an OK packet instead of EOF_Packet
   // & Capabilities::CLIENT_SESSION_TRACK //provides human readable status information & allow server to send the ServerStatus::Server_SESSION_STATE_CHANGED flag
    /*
    & Capabilitites::CLIENT_CONNECT_ATTR
    & Capabilities::CLIENT_MULTI_RESULTS
    & Capabilities::CLIENT_MULTI_STATEMENTS
    & Capabilities::CLIENT_TRANSACTION
    */
}

pub fn server_has_capability(server_handshake: RequestV10, capability: Capabilities) -> bool {
    server_handshake.capabilities.contains(capability)
}