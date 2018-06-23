#[macro_use]
extern crate bitflags;
extern crate uri;
extern crate ring;

use mysql::client::MySqlClient;
use uri::Uri;

pub mod mysql;

pub struct ConnectionInfo {
    uri: Uri,
    _username: String,
    _password: String
}

pub fn get_client(client_type: SupportedClient, server_details: ConnectionInfo) -> Box<DatabaseClient> {
    match client_type {
        SupportedClient::MySQL => Box::new(MySqlClient::new(server_details))
    }
}

pub trait DatabaseClient {
    fn connect(&mut self) -> Result<(),String>;
}

pub enum SupportedClient {
    MySQL
}

pub trait AuthCredentials {
    fn get_credentials() -> String;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
