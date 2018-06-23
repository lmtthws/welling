use std::io::{BufReader, Read};
use std::cmp;

use mysql::packets::Header;
use mysql::packets::ReadFromBuffer;
use mysql::packets::protocol_reader::ProtocolTypeReader;

pub struct RequestV10 {
    pub header: Header,
    pub version: u8,
    pub server_version: String,
    pub thread_id: u32,
    pub char_set: u8,
    pub status_flags: u16,
    pub capabilities: u32,
    pub auth_plugin: Option<AuthPlugin>,

}

pub struct AuthPlugin {
    pub name: String,
    pub auth_data: String,
}

impl ReadFromBuffer for RequestV10 {

    fn read<R: Read>(reader: &mut BufReader<R>) -> Result<RequestV10,String> {
        let header = Header::read(reader)?;

        let protocol_version;
        match reader.next_u8() {
            Ok(v) => protocol_version = v,
            Err(_) => return Err(String::from("Server handshake protocol version not received"))
        }

        if protocol_version != 10 {
           return Err(String::from("Unsupported protocol version"));
        }

        let server_version = reader.next_null_string()?;
        let thread_id = reader.next_u32()?;
        let mut plugin_data = vec!();; //first 8 ASCII chars of scramble - could be ignored based on server capabilties, else we take a slice of auth_data_len from this and at least 13 more bytes...
        plugin_data.extend_from_slice(reader.next_fixed_string(8)?.as_bytes());

        reader.advance(1)?; //filler

        let capabilities = reader.next_u16()?; //lower two bytes

        let char_set = reader.next_u8()?;
        let status_flags = reader.next_u16()?;

        let capabilities = ((reader.next_u16()? as u32) << 16) | (capabilities as u32); //prepend upper two bytes

        let auth_data_len = reader.next_u8()?;

       reader.advance(10)?; //reserved - should be all 0s //TODO: verify all zeros

        let mut auth_plugin : Option<AuthPlugin> = None;
        if 1_u32 == ((capabilities & (1 << 19)) >> 19 as u32) {
            let auth_data_len = cmp::max(13, auth_data_len - 8);
            let mut auth_data_plugin = vec!();
            auth_data_plugin.extend_from_slice(reader.next_fixed_string(auth_data_len)?.as_bytes());
            plugin_data.append(&mut auth_data_plugin);
            plugin_data.truncate(auth_data_len as usize);
            let auth_data: String;
            match String::from_utf8(plugin_data) {
                Ok(s) => auth_data = s,
                Err(s) => return Err(s.to_string())
            }

            let plugin_name = reader.next_null_string()?;
            auth_plugin = Some(AuthPlugin{ name: plugin_name, auth_data })

        } else {
            if b'0' != auth_data_len {return Err(String::from("Invalid auth_plugin_data_len value. Expected 0 due to no stated client plugin auth flag in server capabilities."))}
            reader.advance(13)?;
        }

        Ok(RequestV10 {
            header,
            version: protocol_version,
            server_version,   
            thread_id,
            capabilities,
            char_set,
            status_flags,
            auth_plugin 
        })
    }
}