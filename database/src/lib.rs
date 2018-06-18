#[macro_use]
extern crate bitflags;
extern crate uri;

use mysql::client::MySqlClient;
use uri::Uri;

pub mod mysql;

pub struct ConnectionInfo {
    uri: Uri,
    username: String,
    password: String
}

pub  fn GetClient(clientType: SupportedClient, server_details: ConnectionInfo) -> Box<DatabaseClient> {
    match clientType {
        MYSQL => Box::new(MySqlClient::new(server_details))
    }
}

pub trait DatabaseClient {
    fn Connect(&mut self) -> Result<(),String>;
}

pub enum SupportedClient {
    MYSQL
}

pub trait AuthCredentials {
    fn GetCredentials() -> String;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
