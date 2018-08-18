use ::std::io::{BufWriter, Write};
use mysql::packets::protocol_types::*;
use mysql::packets::bytes::get_bytes;

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

trait ProtocolTypeConverter {
    fn to_length_encoded_string(orig: &str) -> LengthEncodedString;
}

pub trait ProtocolTypeWriter {
    fn write_u8(&mut self, int: u8) -> Result<(),::std::io::Error>;
    fn write_u16(&mut self, int: u16) -> Result<(),::std::io::Error>;
    fn write_u24(&mut self, int: u24) -> Result<(),::std::io::Error>;
    fn write_u32(&mut self, int: u32) -> Result<(),::std::io::Error>;
}

impl<W> ProtocolTypeWriter for BufWriter<W> where W: Write {
    fn write_u8(&mut self, int: u8) -> Result<(),::std::io::Error> {
        write_integer(self, &int.to_fixed_integer())
    }
    fn write_u16(&mut self, int: u16) -> Result<(),::std::io::Error>{
        write_integer(self, &int.to_fixed_integer())
    }
    fn write_u24(&mut self, int: u24) -> Result<(),::std::io::Error>{
        write_integer(self, &int.to_fixed_integer())
    }
    fn write_u32(&mut self, int: u32) -> Result<(),::std::io::Error>{
        write_integer(self, &int.to_fixed_integer())
    }
}

fn write_integer<W: Write>(writer: &mut BufWriter<W>, int: &FixedInteger) -> Result<(),::std::io::Error> {
    write!(writer, "{}", int)?;
    Ok(())
}