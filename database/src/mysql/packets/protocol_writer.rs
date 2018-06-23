use ::std::io::{BufWriter, Write};
use mysql::packets::protocol_types::*;

pub trait ProtocolIntConverter<FixedInteger> {
    fn to_fixed_integer(self) -> FixedInteger;
}

impl ProtocolIntConverter<FixedInteger> for u8 {
    fn to_fixed_integer(self) -> FixedInteger {
        let bytes = get_bytes(self as u32);
        FixedInteger::Int1([bytes[3]])
    }
}

impl ProtocolIntConverter<FixedInteger> for u16 {
    fn to_fixed_integer(self) -> FixedInteger {
        let bytes = get_bytes(self as u32);
        FixedInteger::Int2([bytes[2], bytes[3]])
    }
}

impl ProtocolIntConverter<FixedInteger> for u24 {
    fn to_fixed_integer(self) -> FixedInteger {
        let bytes = get_bytes(self.0 as u32);
        FixedInteger::Int3([bytes[1], bytes[2], bytes[3]])
    }
}

impl ProtocolIntConverter<FixedInteger> for u32 {
    fn to_fixed_integer(self) -> FixedInteger {
        let mut bytes = get_bytes(self as u32);
        bytes.reverse();
        FixedInteger::Int4(bytes)
    }
}

fn get_bytes(num: u32) -> [u8;4] {
    let first : u8 = ((num >> 24) & 0xff) as u8;
    let second : u8 = ((num >> 16) & 0xff) as u8;
    let third : u8 = ((num >> 8) & 0xff) as u8;
    let fourth : u8 = (num & 0xff) as u8;
    [first, second, third, fourth]
}

trait ProtocolTypeConverter {
    fn to_length_encoded_string(orig: &str) -> LengthEncodedString;
}

pub trait ProtocolTypeWriter {
    fn write_u8(&mut self, int: u8) -> Result<(),String>;
    fn write_u16(&mut self, int: u16) -> Result<(),String>;
    fn write_u24(&mut self, int: u24) -> Result<(),String>;
    fn write_u32(&mut self, int: u32) -> Result<(),String>;
}

impl<W> ProtocolTypeWriter for BufWriter<W> where W: Write {
    fn write_u8(&mut self, int: u8) -> Result<(),String> {
        write_integer(self, &int.to_fixed_integer())
    }
    fn write_u16(&mut self, int: u16) -> Result<(),String>{
        write_integer(self, &int.to_fixed_integer())
    }
    fn write_u24(&mut self, int: u24) -> Result<(),String>{
        write_integer(self, &int.to_fixed_integer())
    }
    fn write_u32(&mut self, int: u32) -> Result<(),String>{
        write_integer(self, &int.to_fixed_integer())
    }
}

fn write_integer<W: Write>(writer: &mut BufWriter<W>, int: &FixedInteger) -> Result<(),String> {
        match write!(writer, "{}", int) {
            Err(_) => Err(String::from("Failed to write to output buffer")),
            _ => Ok(())
        }
    }