use std::fmt::Error;
use std::fmt::Formatter;
use std::net::TcpStream;
use std::io::BufWriter;
use std::io::Write;
use std::fmt::Display;

use mysql::packets::protocol_types::*;

//verify Capabilities::client_protocol_41 else should do 320, but our server will support 41 and we support 41, so...
pub struct Response41 {
    pub capabilities: u32,
    pub max_packet_size: u32,
    pub char_set: u8,
    //filler - 23 0s
    pub username: NullTerminatedString, //null-terminated
    pub auth_response: AuthResponse,  //type of auth response determined by CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA capability
    pub init_database: Option<NullTerminatedString>, //if CLIENT_CONNECT_WITH_DB in capabilities - should be in char set from char_set field
    pub auth_plugin_name: Option<NullTerminatedString>, //if CLIENT_PLUGIN_AUTH; this is the method used to generate auth_response value - should be utf8
    pub connection_attributes: Option<ConnectAttributes> //if CLIENT_CONNECT_ATTRS in capabilities
}

impl Response41 {
    pub fn write_to_stream(&self, stream: &mut TcpStream) -> Result<(),::std::io::Error> {
       let mut writer = BufWriter::new(stream);
       
       write!(writer, "{}", self.capabilities)?;
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



pub enum AuthResponse {
    Standard(LengthEncodedString),
    Long(u32,String)
}

impl Display for AuthResponse {
     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
           AuthResponse::Standard(ref ls) => {
                write!(f,"{}", ls)?;
                Ok(())
           },
           AuthResponse::Long(ref l, ref s) => {
                write!(f,"{}{}",l,s)?;
                Ok(())
            }
        }
    }
}

pub struct ConnectAttributes {
    pub nvp: Vec<(LengthEncodedString,LengthEncodedString)>
}

impl ConnectAttributes {
    fn length(&self) -> LengthInteger {
        let mut length: u64 = 0;
        for nv in self.nvp.iter() {
            length += (nv.0).0.total_bytes() + (nv.1).0.total_bytes();
        }

        LengthInteger::new(length)
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