use mysql::packets::{Header, ReadablePacket, WriteablePacket};
use mysql::packets::protocol_reader::ProtocolTypeReader;
use mysql::packets::protocol_types::*;
use ::std::io::{Read, Write, BufReader, BufWriter};

pub struct AuthPlugin {
    pub name: String,
    pub auth_data: String,
}

pub struct AuthSwitchRequest {
    header: Header,
    plugin: AuthPlugin
}

impl AuthSwitchRequest {
    const MIN_SIZE: u64 = 2;

    pub fn plugin(&self) -> &AuthPlugin {
        &self.plugin
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}

impl ReadablePacket for AuthSwitchRequest {
    fn read<R: Read>(reader: &mut BufReader<R>) -> Result<AuthSwitchRequest,String> {
        let header = Header::read(reader)?;
        
        let status = reader.next_u8()?;
        if status != 0xFE {
            return Err(String::from("Identifier in packet was not for an authentication switch"))
        }

        let plugin_name = reader.next_null_string()?;

        let bytes_to_read: u64 = header.packet_len().0 as u64 - AuthSwitchRequest::MIN_SIZE - plugin_name.len() as u64;
        let auth_data = reader.next_fixed_string(bytes_to_read)?;

        Ok(AuthSwitchRequest{
            header,
            plugin: AuthPlugin {
                name: plugin_name,
                auth_data
            }
        })
    }
}

pub struct AuthSwitchResponse {
    header: Header,
    auth_data: String,
}

impl AuthSwitchResponse {
    pub fn new(auth_data: String, sequence_id: u8) -> Result<AuthSwitchResponse,String> {
        if auth_data.len() as u64 > u24::MAX as u64 {
            return Err(String::from("Authentication response data is too large for a single response packet"));
        }
        
        let header = Header::new(u24(auth_data.len() as u32), sequence_id);
        Ok(AuthSwitchResponse {
            header,
            auth_data
        })
    }
}

impl WriteablePacket for AuthSwitchResponse {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(), ::std::io::Error> {
        self.header.write(writer)?;
        write!(writer, "{}", self.auth_data)?;
        Ok(())
    }
}

