pub mod handshake;
pub mod protocol_types;
pub mod general_response;
pub mod protocol_reader;
pub mod protocol_writer;

use std::marker::Sized;
use std::io::{BufReader, Read, BufWriter, Write, ErrorKind};

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
    is_unsized: bool
}

impl Header {
    pub fn new_unsized(sequence_id: u8) -> Header {
        Header {
            payload_length: u24(0),
            sequence_id,
            is_unsized: true
        }
    }

    pub fn new(payload_length: u24, sequence_id: u8) -> Header {
        Header {
            payload_length,
            sequence_id,
            is_unsized: false
        }
    }

    pub fn is_unsized(&self) -> bool {
        self.is_unsized
    }

    pub fn expect_more_packets(&self) -> bool {
        self.payload_length.0 == 0xFFFFFF
    }

    pub fn packet_len(&self) -> u24 {
        self.payload_length
    }

    pub fn sequence_id(&self) -> u8 {
        self.sequence_id
    }
}

impl ReadablePacket for Header {
    fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<Header,String> {
        let payload_length = buffer.next_u24()?;
        let sequence_id = buffer.next_u8()?;

        Ok(Header{
            payload_length,
            sequence_id,
            is_unsized: false,
        })
    }
}

impl WriteablePacket for Header {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error>  {
        if self.is_unsized {
            return Err(::std::io::Error::from(ErrorKind::InvalidInput))
        }
        
        writer.write_u24(self.payload_length)?;
        writer.write_u8(self.sequence_id)?;
        Ok(())
    }
}
