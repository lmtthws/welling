mod request;
mod response;
mod authentication;

pub use self::authentication::{AuthMoreData, AuthSwitchRequest, AuthSwitchResponse, AuthPlugin};
pub use self::request::RequestV10;
pub use self::response::Response41;

use ::std::io::{Read, BufReader};
use crate::mysql::packets::{Header, ReadablePacket};
use crate::mysql::packets::general_response::{OkPacket41, ErrPacket41};
use crate::mysql::packets::protocol_reader::ProtocolTypeReader;

pub enum HandshakeServerResponse {
    Okay(OkPacket41),
    Error(ErrPacket41),
    AuthSwitch(AuthSwitchRequest),
    MoreAuthData(AuthMoreData)
}

impl ReadablePacket for HandshakeServerResponse {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<HandshakeServerResponse, String> {
        let identifier = buffer.next_u8()?;

        match identifier {
            0x00 => Ok(HandshakeServerResponse::Okay(OkPacket41::read(buffer, header)?)),
            0xFF => Ok(HandshakeServerResponse::Error(ErrPacket41::read(buffer, header)?)),
            0x01 => Ok(HandshakeServerResponse::MoreAuthData(AuthMoreData::read(buffer, header)?)),
            0xFE => Ok(HandshakeServerResponse::AuthSwitch(AuthSwitchRequest::read(buffer, header)?)),
            _ => Err(format!("Unexpected identifier received in server's response to handshake response: {}", identifier))
        }
    }
}