use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;

use mysql::packets::protocol_types::u24;

trait ProtocolTypeConverter {
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u24(&self) -> u24;
    fn to_u32(&self) -> u32;
    fn to_pstring(&self) -> Result<String,String>;
}


impl ProtocolTypeConverter for Vec<u8> {
    fn to_u8(&self) -> u8 {
        self[0]
    }

    fn to_u16(&self) -> u16 {
        let temp: Vec<u16> = self.into_iter().map(|x| *x as u16).collect();
        temp[1] << 8 | temp[0]
    }

    fn to_u24(&self) -> u24 {
        let temp: Vec<u32> = self.into_iter().map(|x| *x as u32).collect();
        u24((temp[2] << 16) | (temp[1] << 8) | temp[0])
    }

    fn to_u32(&self) -> u32 {
         let temp: Vec<u32> = self.into_iter().map(|x| *x as u32).collect();
        (temp[3] << 24) | (temp[2] << 16) | (temp[1] << 8) | temp[0]
    }

    fn to_pstring(&self) -> Result<String,String> {
        match String::from_utf8(self.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(String::from("Expected a string of ASCII bytes, but invalid sequence was received."))
        }
    }
}

pub trait ProtocolTypeReader{
    fn advance(&mut self, len: u8) ->Result<(),String>;

    fn next_u8(&mut self) -> Result<u8,String> ;
    fn next_u16(&mut self) -> Result<u16,String> ;
    fn next_u24(&mut self) -> Result<u24,String>;
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

    fn next_u24(&mut self) -> Result<u24,String> {
        let val = read_exact(self, 3)?.to_u24();
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