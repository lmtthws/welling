use std::fmt::Error;
use std::fmt::Formatter;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::cmp;
use std::fmt::Display;



pub struct RequestV10 {
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

impl RequestV10 {

    pub fn read(stream: &mut TcpStream) -> Result<RequestV10,String> {
        let mut reader = BufReader::new(stream);

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
        let mut plugin_data = read_exact(&mut reader, 8)?; //first 8 ASCII chars of scramble - could be ignored based on server capabilties, else we take a slice of auth_data_len from this and at least 13 more bytes...

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
            let mut auth_data_plugin = read_exact(&mut reader, auth_data_len)?;
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

trait ProtocolTypeConverter {
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u32(&self) -> u32;
    fn to_pstring(&self) -> Result<String,String>;
}

trait ProtocolTypeReader{

    fn advance(&mut self, len: u8) ->Result<(),String>;

    fn next_u8(&mut self) -> Result<u8,String> ;
    fn next_u16(&mut self) -> Result<u16,String> ;
    fn next_u32(&mut self) -> Result<u32,String> ;
    fn next_fixed_string(&mut self, len: u8) -> Result<String,String> ;
    fn next_null_string(&mut self) -> Result<String,String> ;
}

impl<R> ProtocolTypeReader for BufReader<R> where R: ::std::io::Read {
    fn advance(&mut self, len: u8) -> Result<(), String> {
        match read_exact(self, len) {
            Ok(_) => Ok(()),
            Err(s) => Err(s)
        }
    }

    fn next_u8(&mut self) -> Result<u8,String> {
        let val = read_exact(self, 1)?.to_u8();
        Ok(val)
    }
    fn next_u16(&mut self) -> Result<u16,String>  {
        let val = read_exact(self, 2)?.to_u16();
        Ok(val)
    }
    fn next_u32(&mut self) -> Result<u32,String> {
        let val = read_exact(self, 4)?.to_u32();
        Ok(val)
    }

    fn next_fixed_string(&mut self, len: u8) -> Result<String,String> {
       read_exact(self, len)?.to_pstring()
    }

    fn next_null_string(&mut self) -> Result<String,String> {
        let mut next_string: Vec<u8> = Vec::new();
        match self.read_until(b'0', &mut next_string) {
            Ok(_) => (),
            Err(_) => return Err(String::from("Null-terminated string field expected but no terminator was supplied"))
        }
        
       next_string.to_pstring()
    }
}

fn read_exact<R>(reader: &mut BufReader<R>, count: u8) -> Result<Vec<u8>,String> where R: ::std::io::Read {
    let count = count as usize;
    let buffer = Vec::with_capacity(count);
    let mut buffer = buffer.into_boxed_slice();

    match reader.read(&mut buffer) {
        Ok(n) => if n != count {
            return Err(String::from("Packet field was not expected length"))
        },
        Err(_) => {
            return Err(String::from("Error reading field value in handshake"))
        }
    }

    Ok(buffer.to_vec())
}


impl ProtocolTypeConverter for Vec<u8> {
    fn to_u8(&self) -> u8 {
        self[0]
    }

    fn to_u16(&self) -> u16 {
        let temp: Vec<u16> = self.into_iter().map(|x| *x as u16).collect();
        (temp[0] << 8) | temp[1]
    }

    fn to_u32(&self) -> u32 {
         let temp: Vec<u32> = self.into_iter().map(|x| *x as u32).collect();
        (temp[0] << 24) | temp[1] << 16 | temp[2] << 8 | temp[3]
    }

    fn to_pstring(&self) -> Result<String,String> {
        match String::from_utf8(self.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(String::from("Expected a string of ASCII bytes, but invalid sequence was received."))
        }
    }
}



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

#[derive(Clone,Copy)]
pub enum LengthInteger {
    U8(u8),
    U16(u16), //prefixed with 0xFC
    U24(u32),
    U64(u64) //prefixed with 0xFe
}

impl LengthInteger {
    fn new(integer: u64) -> LengthInteger {
        if integer < 251 {
            LengthInteger::U8(integer as u8)
        } else if integer < (1<<16) {
            LengthInteger::U16(integer as u16)
        } else if integer < (1<<24) {
            LengthInteger::U24(integer as u32)
        } else {
           LengthInteger::U64(integer)
        }
    }
    
    fn total_bytes(&self) -> u64 {
        match *self {
            LengthInteger::U8(i) => 1 + i as u64,
            LengthInteger::U16(i) => 3 + i as u64,
            LengthInteger::U24(i) => 4 + i as u64,
            LengthInteger::U64(i) => 9 + i
        }
    }
}

impl Display for LengthInteger {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
       match *self {
            LengthInteger::U8(i) => write!(f,"{}",i)?,
            LengthInteger::U16(i) => write!(f,"{}{}",0xFC,i)?,
            LengthInteger::U24(i) => write!(f,"{}{}",0xFD,i)?,
            LengthInteger::U64(i) => write!(f,"{}{}",0xFE,i)?
        };
        Ok(())
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

#[derive(Clone)]
pub struct LengthEncodedString(LengthInteger,String);

impl Display for LengthEncodedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}{}", self.0, self.1)?;
        Ok(())
    }
}

pub struct NullTerminatedString(String);

impl Display for NullTerminatedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}{}", self.0, '\0')?;
        Ok(())
    }
}