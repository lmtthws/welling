pub mod handshake;
pub mod protocol_types;
pub mod general_response;
pub mod protocol_reader;
pub mod protocol_writer;

use std::marker::Sized;
use std::io::{BufReader, Read, BufWriter, Write};

use self::protocol_types::u24;
use self::protocol_reader::ProtocolTypeReader;
use self::protocol_writer::ProtocolTypeWriter;




pub trait ReadablePacket {
    fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<Self,String> where Self: Sized;
}
pub trait WriteablePacket {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error>;
}



pub struct Header {
    payload_length: u24,
    sequence_id: u8,

}

impl Header {
    pub fn expect_more_packets(&self) -> bool {
        self.payload_length.0 == 0xFFFFFF
    }

    pub fn packet_len(&self) -> u24 {
        self.payload_length
    }
}

impl ReadablePacket for Header {
    fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<Header,String> {
        let payload_length = buffer.next_u24()?;
        let sequence_id = buffer.next_u8()?;

        Ok(Header{
            payload_length,
            sequence_id
        })
    }
}

impl WriteablePacket for Header {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error>  {
        writer.write_u24(self.payload_length)?;
        writer.write_u8(self.sequence_id)?;
        Ok(())
    }
}
