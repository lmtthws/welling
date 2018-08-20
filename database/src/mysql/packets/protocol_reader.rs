use std::io::{BufReader, BufRead, Read};
use mysql::packets::protocol_types::*;
use mysql::packets::bytes::Endian;

//TODO: make the buf reader operations safer - if the stream terminates or the packet is not well formed, things may not go well, especially for read_exact



pub trait ProtocolTypeConverter {
    fn to_u8(&self) -> u8;
    fn to_u16(&self, order: Endian) -> u16;
    fn to_u24(&self, order: Endian) -> u24;
    fn to_u32(&self, order: Endian) -> u32;
    fn to_u64(&self, order: Endian) -> u64;
    fn to_pstring(&self) -> Result<String,String>;
}

impl ProtocolTypeConverter for Vec<u8> {
    fn to_u8(&self) -> u8 {
        self[0]
    }

    fn to_u16(&self, order: Endian) -> u16 {
        let temp: Vec<u16> = self.into_iter().map(|x| *x as u16).collect();
        match order {
            Endian::Little => (temp[1] << 8 | temp[0]) & 0xFFFF,
            Endian::Big => (temp[0] << 8 | temp[1]) & 0xFFFF
        }
    }

    fn to_u24(&self, order: Endian) -> u24 {
        let temp: Vec<u32> = self.into_iter().map(|x| *x as u32).collect();
        match order {
             Endian::Little => u24(((temp[2] << 16) | (temp[1] << 8) | temp[0]) & 0xFFFF_FF),
             Endian::Big =>  u24(((temp[0] << 16) | (temp[1] << 8) | temp[2]) & 0xFFFF_FF)
        }
    }

    fn to_u32(&self, order: Endian) -> u32 {
        let temp: Vec<u32> = self.into_iter().map(|x| *x as u32).collect();
        match order {
            Endian::Little => ((temp[3] << 24) | (temp[2] << 16) | (temp[1] << 8) | temp[0]) & 0xFFFF_FFFF,
            Endian::Big => ((temp[0] << 24) | (temp[1] << 16) | (temp[2] << 8) | temp[3]) & 0xFFFF_FFFF,
        }
    }

    fn to_u64(&self, order: Endian) -> u64 {
        let temp: Vec<u64> = self.into_iter().map(|x| *x as u64).collect();
        match order {
            Endian::Little => ((temp[7] << 56) | (temp[6] << 48) | (temp[5] << 40) | temp[4] << 32 | temp[3] << 24 | temp[2] << 16 | temp[1] << 8 | temp[0]) & 0xFFFF_FFFF_FFFF_FFFF,
            Endian::Big => ((temp[0] << 56) | (temp[1] << 48) | (temp[2] << 40) | temp[3] << 32 | temp[4] << 24 | temp[5] << 16 | temp[6] << 8 | temp[7]) & 0xFFFF_FFFF_FFFF_FFFF,
        }
    }

    fn to_pstring(&self) -> Result<String,String> {
        match String::from_utf8(self.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Err(String::from("Expected a string of ASCII bytes, but invalid sequence was received."))
        }
    }
}

pub trait ProtocolTypeReader where Self: Sized {
    fn advance(&mut self, len: u8) ->Result<(),String>;

    fn next_u8(&mut self) -> Result<u8,String> ;
    fn next_u16(&mut self) -> Result<u16,String>;
    fn next_u24(&mut self) -> Result<u24,String>;
    fn next_u32(&mut self) -> Result<u32,String>;
    fn next_u64(&mut self) -> Result<u64,String>;
    fn next_fixed_string(&mut self, len: u64) -> Result<String,String>;
    fn next_null_string(&mut self) -> Result<String,String>;

    fn next_length_integer(&mut self) -> Result<LengthInteger,String>;
    fn next_length_string(&mut self) -> Result<LengthEncodedString,String>;
}

impl<R> ProtocolTypeReader for BufReader<R> where R: ::std::io::Read { //we can optimize this based on max packet size (16mb or u24)
    fn advance(&mut self, len: u8) -> Result<(), String> {
        match read_exact(self, len as u32) {
            Ok(_) => Ok(()),
            Err(s) => Err(s)
        }
    }

    fn next_u8(&mut self) -> Result<u8,String> {
        let val = read_exact(self, 1)?.to_u8();
        Ok(val)
    }
    fn next_u16(&mut self) -> Result<u16,String>  {
        let val = read_exact(self, 2)?.to_u16(Endian::Little);
        Ok(val)
    }

    fn next_u24(&mut self) -> Result<u24,String> {
        let val = read_exact(self, 3)?.to_u24(Endian::Little);
        Ok(val)
    }

    fn next_u32(&mut self) -> Result<u32,String> {
        let val = read_exact(self, 4)?.to_u32(Endian::Little);
        Ok(val)
    }

    fn next_u64(&mut self) -> Result<u64,String> {
        let val = read_exact(self, 8)?.to_u64(Endian::Little);
        Ok(val)
    }

    fn next_fixed_string(&mut self, len: u64) -> Result<String,String> {
        let mut handle = self.take(len);
        let mut string: Vec<u8> = Vec::new();
        match handle.read_to_end(&mut string) {
            Err(e) => return Err(format!("{}",e)),
            _ => ()
        }
        string.to_pstring()
    }

    fn next_null_string(&mut self) -> Result<String,String> {
        let mut next_string: Vec<u8> = Vec::new();
        match self.read_until(b'0', &mut next_string) {
            Ok(_) => (),
            Err(_) => return Err(String::from("Null-terminated string field expected but no terminator was supplied"))
        }
        
       next_string.to_pstring()
    }

    fn next_length_integer(&mut self) -> Result<LengthInteger,String> {
        let length = self.next_u8()?;
        let val: u64;
        match length {
            LengthInteger::TWO_BYTE_PREFIX => val = read_exact(self, 2)?.to_u16(Endian::Big) as u64,
            LengthInteger::THREE_BYTE_PREFIX => val = read_exact(self, 3)?.to_u24(Endian::Big).0 as u64,
            LengthInteger::EIGHT_BYTE_PREFIX => val = read_exact(self, 8)?.to_u64(Endian::Big),
            0xFB | 0xFF => return Err(format!("Expected fixed length integer, but first byte was invalid ({})", length)),
            _ => val = length as u64
        }

        Ok(LengthInteger::new(val))
    }

    fn next_length_string(&mut self) -> Result<LengthEncodedString,String> {
        let length = self.next_length_integer()?;
        let string = self.next_fixed_string(length.value())?;

        Ok(LengthEncodedString::from_unchecked(length,string))
    }
}

pub fn read_exact<R>(reader: &mut BufReader<R>, count: u32) -> Result<Vec<u8>,String> where R: ::std::io::Read {
    if count == 0 {
        return Ok(vec!())
    }
    
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