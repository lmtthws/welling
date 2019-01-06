use std::convert::From;
use std::fmt::{Display, Formatter};
use log::debug;
use crate::mysql::packets;
use crate::mysql::packets::handshake::AuthPlugin;
use crate::{ConnectionInfo};

pub enum SupportedAuthMethods {
    Unknown,
    MySQLOldPassword,
    MySQLNativePassword
}

pub struct AuthenticationResponse {
    method: SupportedAuthMethods,
    data: String,
}

impl AuthenticationResponse {
    pub fn new(method: SupportedAuthMethods, data: String) -> AuthenticationResponse {
        AuthenticationResponse {
            method, data
        }
    }

    pub fn method(&self) -> &SupportedAuthMethods {
        &self.method
    }

    pub fn data(&self) -> String {
        String::from(&*self.data)
    }
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
    pub fn default() -> SupportedAuthMethods {
        SupportedAuthMethods::MySQLOldPassword
    }

    pub fn get_auth_response_value(&self, connection_info: &ConnectionInfo, auth_data: &Option<&AuthPlugin> ) -> Result<String,String> {
        match *self {
            SupportedAuthMethods::Unknown => Err(String::from("should force disconnect")), //TODO: force disconnect...
            SupportedAuthMethods::MySQLOldPassword => {
                let hash_bytes = old_password_hash(&connection_info.password);
                Ok(String::from_utf8(hash_bytes.to_vec()).unwrap())
            },
            SupportedAuthMethods::MySQLNativePassword => {
                if let Some(ref auth_data) = auth_data {
                    let hash_bytes = native_password_hash(&connection_info.password, &auth_data.auth_data)?;
                    Ok(String::from_utf8(hash_bytes.to_vec()).unwrap())
                } else {
                    Err(String::from("Cannot perform native password authentication without data from the server"))
                }
            }
        }
    }
}

#[allow(dead_code)]
fn old_password_hash(password: &str) -> [u8;8] {
    let mut nr: u64  = 1_345_345_333;
    let mut add: u64 = 7;
    let mut nr2: u64 = 0x1234_5671;

    for c in password.chars() { 
        //Python impl: https://djangosnippets.org/snippets/1508/ - 
        // that uses Code Points for each value in a string, but that covers 0 to 0x10FFFF, while Rust char covers 0 to 0xD7FF and 0xE000 to 0x10FFF - 
        // so we miss 2304 potential values. I don't know enough about unicode to really get this - it's probably fine for your general ASCII-type passwords, but maybe someone was using emojis
        let c = c as u64;
        debug!("{}",c);
        nr = nr ^ ((((nr & 63) + add) * c) + (nr << 8) & 0xFFFF_FFFF);
        debug!("{}",nr);
        nr2 = (nr2 + ((nr2 << 8) ^ nr)) & 0xFFFF_FFFF;
        debug!("{}",nr2);
        add = (add + c) & 0xFFFF_FFFF;
        debug!("{}",add);
    }

    nr = nr & 0x7FFF_FFFF;
    debug!("{}",nr);
    nr2 = nr2 & 0x7FFF_FFFF;
    debug!("{}",nr2);

    let mut hash_bytes = [0_u8;8];
    hash_bytes[0..4].clone_from_slice(&packets::get_bytes(nr as u32));
    hash_bytes[4..8].clone_from_slice(&packets::get_bytes(nr2 as u32));

    hash_bytes
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

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn old_password_matches_expected_bananas() {
        let _ = simple_logger::init();

        debug!("Hello from your test");

        let expected: [u8;8] = [96, 24, 120, 130, 46, 198, 121, 209];
        let hash = old_password_hash("Bananas"); 
        
        assert_eq!(hash,expected); //"601878822ec679d1" - output from py code - hex
    }

    #[test]
    fn old_password_matches_expected_unicode() {
        let _ = simple_logger::init();

        let expected: [u8;8] = [7, 111, 97, 159, 73, 8, 137, 121];
        let hash = old_password_hash("y̆"); 
        
        assert_eq!(hash,expected); //"07 6f 61 9f 49 08 89 79" - output from py code
    }
}