use crate::mysql::packets::{Header, ReadablePacket, WriteablePacket};
use crate::mysql::packets::protocol_reader::ProtocolTypeReader;
use crate::mysql::packets::protocol_types::*;
use ::std::io::{Read, Write, BufReader, BufWriter};

pub struct AuthPlugin {
    pub name: String,
    pub auth_data: String,
}

#[allow(dead_code)]
pub struct AuthMoreData {
    more_data: String
}

impl ReadablePacket for AuthMoreData {
     fn read<R: Read>(reader: &mut BufReader<R>, header: &Header) -> Result<AuthMoreData,String> {
        let more_data = reader.next_fixed_string(u64::from(header.packet_len()))?;
        Ok(AuthMoreData{ more_data })
     }
}


pub struct AuthSwitchRequest {
    plugin: AuthPlugin
}

impl AuthSwitchRequest {
    const MIN_SIZE: u64 = 2;

    pub fn plugin(&self) -> &AuthPlugin {
        &self.plugin
    }
}


impl ReadablePacket for AuthSwitchRequest {
    fn read<R: Read>(reader: &mut BufReader<R>, header: &Header) -> Result<AuthSwitchRequest,String> {
        let plugin_name = reader.next_null_string()?;

        let bytes_to_read: u64 = header.packet_len().0 as u64 - AuthSwitchRequest::MIN_SIZE - plugin_name.len() as u64;
        let auth_data = reader.next_fixed_string(bytes_to_read)?;

        Ok(AuthSwitchRequest{
            plugin: AuthPlugin {
                name: plugin_name,
                auth_data
            }
        })
    }
}




pub struct AuthSwitchResponse {
    auth_data: String,
}

impl AuthSwitchResponse {
    pub fn new(auth_data: String) -> AuthSwitchResponse {
        AuthSwitchResponse{ auth_data }
    }
}


impl WriteablePacket for AuthSwitchResponse {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(), ::std::io::Error> {
        write!(writer, "{}", self.auth_data)?;
        Ok(())
    }

    fn calculate_header_size(&self) -> Result<u24, String> {
        if self.auth_data.len() as u64 > u24::MAX as u64 {
            Err(String::from("Authentication response data is too large for a single response packet"))
        } else {
            Ok(u24(self.auth_data.len() as u32))
        }
    }
}

