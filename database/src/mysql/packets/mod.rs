pub mod handshake;
mod protocol_types;
mod general_response;
mod protocol_reader;
mod protocol_writer;
mod bytes;
pub mod command;

pub use self::protocol_reader::ProtocolTypeReader;
pub use self::protocol_writer::ProtocolTypeWriter;
pub use self::general_response::*;
pub use self::protocol_types::*;
pub use self::bytes::get_bytes;

use std::marker::Sized;
use std::io::{BufReader, Read, BufWriter, Write};


pub struct ServerPacket<RP: ReadablePacket> {
    header: Header,
    payload: RP
}

impl<RP: ReadablePacket> ServerPacket<RP> {
    pub fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<Self, String> where Self: Sized {
        let payload_length = buffer.next_u24()?;
        let sequence_id = buffer.next_u8()?;

        let header = Header::new(payload_length, sequence_id);
        let payload = RP::read(buffer, &header)?;

        Ok(ServerPacket{header, payload})
    }

    pub fn sequence_id(&self) -> u8 {
        self.header.sequence_id
    }

    pub fn payload(&self) -> &RP {
        &self.payload
    }

    pub fn into_payload(self) -> RP {
        self.payload
    }
}


pub struct ClientPacket<WP: WriteablePacket> {
    header: Header,
    packet: WP
}

impl<WP: WriteablePacket> ClientPacket<WP> {
    pub fn new(packet: WP, sequence_id: u8) -> Result<ClientPacket<WP>,String> {
        let header = Header {
            payload_length: packet.calculate_header_size()?,
            sequence_id
        };
        Ok(ClientPacket { header, packet })
    }

    pub fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error>  { 
        writer.write_u24(self.header.packet_len())?;
        writer.write_u8(self.header.sequence_id())?;
        self.packet.write(writer)?;
        Ok(())
    }
}



pub trait ReadablePacket {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<Self,String> where Self: Sized;
}

pub trait WriteablePacket {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error>;
    fn calculate_header_size(&self) -> Result<u24, String>;
}

pub struct Header {
    payload_length: u24,
    sequence_id: u8,
}

impl Header {
    pub fn new(payload_length: u24, sequence_id: u8) -> Header {
        Header {
            payload_length,
            sequence_id,
        }
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


