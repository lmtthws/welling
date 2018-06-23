pub mod handshake;
pub mod protocol_types;
pub mod generic;
pub mod protocol_reader;
pub mod protocol_writer;

use std::marker::Sized;
use std::io::BufReader;
use std::io::Read;
use std::io::BufWriter;
use std::io::Write;

use self::protocol_types::u24;
use self::protocol_reader::ProtocolTypeReader;
use self::protocol_writer::ProtocolTypeWriter;




pub trait ReadFromBuffer {
    fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<Self,String> where Self: Sized;
}
pub trait WriteToBuffer {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),String>;
}



pub struct Header {
    payload_length: u24,
    sequence_id: u8,

}

impl Header {
    pub fn expect_more_packets(&self) -> bool {
        self.payload_length.0 == 0xFFFFFF
    }
}

impl ReadFromBuffer for Header {
    fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<Header,String> {
        let payload_length = buffer.next_u24()?;
        let sequence_id = buffer.next_u8()?;

        Ok(Header{
            payload_length,
            sequence_id
        })
    }
}

impl WriteToBuffer for Header {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),String>  {
        writer.write_u24(self.payload_length)?;
        writer.write_u8(self.sequence_id)?;
        Ok(())
    }
}
