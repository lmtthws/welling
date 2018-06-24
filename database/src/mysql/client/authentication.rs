extern crate ring;

use mysql::packets::handshake::request::AuthPlugin;
use std::convert::From;
use std::fmt::{Display, Formatter};
use {ConnectionInfo};


pub enum SupportedAuthMethods {
    Unknown,
    MySQLOldPassword,
    MySQLNativePassword
}

impl Display for SupportedAuthMethods {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            SupportedAuthMethods::MySQLOldPassword => write!(f,"mysql_old_password"),
            SupportedAuthMethods::MySQLNativePassword => write!(f,"mysql_native_password"),
            SupportedAuthMethods::Unknown => write!(f,"")
        }
    }
}

impl<'a> From<&'a str> for SupportedAuthMethods {
    fn from(s: &'a str) -> Self {
        match s {
            "mysql_old_password" => SupportedAuthMethods::MySQLOldPassword,
            "mysql_native_password" => SupportedAuthMethods::MySQLNativePassword,
            _ => SupportedAuthMethods::Unknown
        }
    }
}

impl SupportedAuthMethods {

    pub fn get_auth_response_value(&self, connection_info: &ConnectionInfo, auth_data: &AuthPlugin ) -> Result<String,String> {
        match *self {
            SupportedAuthMethods::Unknown => Err(String::from("should force disconnect")), //TODO: force disconnect...
            SupportedAuthMethods::MySQLOldPassword => {Err(String::from("not yet implemented"))},
            SupportedAuthMethods::MySQLNativePassword => {
                let hash_bytes = native_password_hash(&connection_info.password, &auth_data.auth_data)?;
                Ok(String::from_utf8(hash_bytes.to_vec()).unwrap())
            }
        }
    }
}


fn native_password_hash(password: &str, auth_data: &str) -> Result<[u8;20],String> {
    let pass_hash = ring::digest::digest(&ring::digest::SHA1, password.as_bytes());
    let pass_hash = pass_hash.as_ref();
    
    
    let mut server_bytes: Vec<u8> = vec!(); 
    
    let server_auth_data = auth_data.as_bytes();
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
    let mut auth_data: [u8;20] = [0; 20];
    for (ix, (u1,u2)) in pass_hash.into_iter().zip(server_bytes).enumerate() {
        auth_data[ix] = u1 ^ u2;
    }

    return Ok(auth_data);
}