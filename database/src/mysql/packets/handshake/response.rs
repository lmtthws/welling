use std::fmt::Error;
use std::fmt::Formatter;
use std::io::{BufWriter, Write};
use std::fmt::Display;
use mysql::client::capabilities::Capabilities;
use mysql::packets::Header;
use mysql::packets::WriteablePacket;
use mysql::packets::protocol_types::*;

//verify Capabilities::client_protocol_41 else should do 320, but our server will support 41 and we support 41, so...
pub struct Response41 {
    pub header: Header,
    pub capabilities: Capabilities,
    pub max_packet_size: u32,
    pub char_set: u8,
    //filler - 23 0s
    pub username: NullTerminatedString, //null-terminated
    pub auth_response: LengthEncodedString,  //type of auth response determined by CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA capability - capability allows all lengths, else this is a u8+string
    pub init_database: Option<NullTerminatedString>, //if CLIENT_CONNECT_WITH_DB in capabilities - should be in char set from char_set field
    pub auth_plugin_name: Option<NullTerminatedString>, //if CLIENT_PLUGIN_AUTH; this is the method used to generate auth_response value - should be utf8
    pub connection_attributes: Option<ConnectAttributes> //if CLIENT_CONNECT_ATTRS in capabilities
}

impl Response41 {
    pub const MINIMUM_SIZE: u32 = 4 + 4 + 1;

    pub fn calculate_header_size(&mut self) -> Result<(),String> {
        
        let mut size = Response41::MINIMUM_SIZE as u64;
        size += self.username.packet_size();
        size += self.auth_response.packet_size();
        if let Some(ref db) = self.init_database { size += db.packet_size(); }
        if let Some(ref plug) = self.auth_plugin_name { size += plug.packet_size(); }
        if let Some(ref attr) = self.connection_attributes { size += attr.packet_size(); }
        
        if size > u24::MAX as u64 {
            return Err(String::from("Handshake response exceeds maximum response packet length"))
        }

        self.header = Header::new(u24(size as u32), self.header.sequence_id());
        Ok(())
    }
}

impl WriteablePacket for Response41 {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error> {
    
       self.header.write(writer)?;
       
       write!(writer, "{}", self.capabilities.bits())?;
       write!(writer, "{}", self.max_packet_size)?;
       write!(writer, "{}", self.char_set)?;
       for _ in 0..23 {
            write!(writer, "{}",b'0')?
       }
       write!(writer, "{}", self.username)?;
       write!(writer, "{}", self.auth_response)?;
       if let Some(ref ns) = self.init_database {
           write!(writer, "{}", ns)?;
       }
       if let Some(ref ns) = self.auth_plugin_name {
           write!(writer, "{}", ns)?;
       }
       if let Some(ref attr) = self.connection_attributes {
           write!(writer, "{}", attr)?;
       }

       Ok(())
    }
}

pub struct ConnectAttributes {
    pub nvp: Vec<(LengthEncodedString,LengthEncodedString)>
}

impl ConnectAttributes {
    fn length(&self) -> LengthInteger {
        LengthInteger::new(self.packet_size())
    }

    pub fn packet_size(&self) -> u64 {
        let mut length: u64 = 0;
        for nv in self.nvp.iter() {
            length += (nv.0).packet_size() + (nv.1).packet_size();
        }
        length
    }
}

impl Display for ConnectAttributes {
     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}",self.length())?;
        for nv in self.nvp.iter() {
            write!(f,"{}{}",nv.0, nv.1)?;
        }
        Ok(())
    }
}