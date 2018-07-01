extern crate bitflags;

use ::std::io::{BufReader, Read,};
use ::std::net::TcpStream;
use mysql::packets::{Header, ReadablePacket};
use mysql::packets::protocol_types::*;
use mysql::packets::protocol_reader::ProtocolTypeReader;




pub enum Responses{
    Okay(OkPacket41),
    Error(ErrPacket41)
}

impl Responses {
    //TODO: BufReader seek supposedly discards the buffer, which means, I think, that the packet reads will fail...
    pub fn read(stream: &mut TcpStream) -> Result<Responses, String> {
        let mut buffer = [0_u8; 5];
        match stream.peek(&mut buffer) {
            Err(e) => Err(format!("{}",e)),
            Ok(5) => match buffer[4] {
                0x00 | 0xFE => {
                    Ok(Responses::Okay(OkPacket41::read(&mut BufReader::new(stream))?))
                },
                0xFF => {
                    Ok(Responses::Error(ErrPacket41::read(&mut BufReader::new(stream))?))
                },
                _ => Err(format!("Unexpected response packet identifier: {}", buffer[4]))
            },
            _ => Err(format!("Could not read enough bytes to determine server's response packet type"))
        }
    }
}

#[allow(dead_code)]
pub struct OkPacket41 {
    header: Header,
    identifier: u8,
    affected_rows: LengthInteger,
    last_insert_id: LengthInteger,
    status_flags: ServerStatus,
    warnings: u16,
    session_info: Option<LengthEncodedString>, //may be present if CLIENT_SESSION_TRACK capability stated
    server_session_state: Option<LengthEncodedString>, //mutually exclusive with status_info - CLientsessionTrack (so server sends state) + Server_SESSION_STATE_CHANGED
    // also, the above is actually an enum type (0 -> 5: see ref: https://github.com/mysql/mysql-server/blob/8.0/include/mysql_com.h) 
    //                               + string<lenenc> where the string<lenenc> may itself encode 2 string<lenenc>
   
    status_info: Option<String> //rest of packet...
}

impl OkPacket41 {
    const MINIMUM_SIZE: u32 = 1 + 1 + 1 + 2 + 2; //id rows(>1) insert(>1) status(2) warnings(2)
}

impl ReadablePacket for OkPacket41 {
    fn read<R: Read>(reader: &mut BufReader<R>) -> Result<OkPacket41, String> {
        let header = Header::read(reader)?;

        let identifier = reader.next_u8()?;
        match identifier {
            0x00 => (),
            0xFE => (), //maybe do packet size checking?
            _ => return Err(String::from("OK packet identifying byte was not present"))
        }

        let affected_rows = reader.next_length_integer()?;
        let last_insert_id = reader.next_length_integer()?;

        let status_flags_raw = reader.next_u16()?;
        let mut status_flags: ServerStatus = ServerStatus::empty();
        if let Some(f) = ServerStatus::from_bits(status_flags_raw) {
            status_flags = f;
        }

        let warnings = reader.next_u16()?;

        let session_info = Option::None; //TODO: update if support CLIENT_SESSION_TRACK
        
        let mut server_session_state = Option::None;
        let mut status_info = Option::None;
        if status_flags.intersects(ServerStatus::SERVER_SESSION_STATE_CHANGED) {
            server_session_state = Some(reader.next_length_string()?);
        } else {
            let mut bytes_left = header.payload_length.0;
            bytes_left -= OkPacket41::MINIMUM_SIZE;

            bytes_left -= (affected_rows.total_bytes() as u32) - 1; //this is safe, because the max packet size ensures we'd only have u24 here...could differ after we support multiple packet responses...
            bytes_left -= (last_insert_id.total_bytes() as u32) - 1;

            status_info = Some(reader.next_fixed_string(bytes_left as u64)?);
        }

        Ok(OkPacket41{
            header,
            identifier,
            affected_rows,
            last_insert_id,
            status_flags,
            warnings,
            session_info,
            server_session_state, //this is more complicated than currently implemented
            status_info
        })
    }
}

#[allow(dead_code)]
pub struct ErrPacket41 {
    header: Header,
    identifier: u8,
    error_code: u16,
    state_marker: String, //fixed length: 1
    state: String, //fixed length 5
    error_message: String
}

impl ErrPacket41 {
    const MINIMUM_SIZE: u8 = 1 + 2 + 1 + 5;

    pub fn error_code(&self) -> u16 {
        self.error_code
    }
    pub fn error_message(&self) -> &str {
        &*self.error_message
    }
}

impl ReadablePacket for ErrPacket41 {
    fn read<R: Read>(reader: &mut BufReader<R>) -> Result<ErrPacket41,String> {
        let header = Header::read(reader)?;

        let identifier = reader.next_u8()?;
        if identifier != 0xFF {
            return Err(String::from("Error packet identifier was not expected value"))
        }

        let error_code = reader.next_u16()?;
        let state_marker = reader.next_fixed_string(1)?;
        let state = reader.next_fixed_string(5)?;

        let bytes_left = header.packet_len().0 - ErrPacket41::MINIMUM_SIZE as u32;
        let error_message = reader.next_fixed_string(bytes_left as u64)?;

        Ok(ErrPacket41{
            header,
            identifier,
            error_code,
            state_marker,
            state,
            error_message
        })
    }
}


bitflags! {
    pub struct ServerStatus: u16 {     //ref: https://github.com/mysql/mysql-server/blob/8.0/include/mysql_com.h
        const SERVER_STATUS_IN_TRANS  = 1;
        const SERVER_STATUS_AUTOCOMMIT  = 2;
        const SERVER_MORE_RESULTS_EXISTS  = 8;
        const SERVER_QUERY_NO_GOOD_INDEX_USED  = 16;
        const SERVER_QUERY_NO_INDEX_USED  = 32;
        const SERVER_STATUS_CURSOR_EXISTS  = 64;
        const SERVER_STATUS_LAST_ROW_SENT  = 128;
        const SERVER_STATUS_DB_DROPPED  = 256;
        const SERVER_STATUS_NO_BACKSLASH_ESCAPES  = 512;
        const SERVER_STATUS_METADATA_CHANGED  = 1024;
        const SERVER_QUERY_WAS_SLOW  = 2048;
        const SERVER_PS_OUT_PARAMS  = 4096;
        const SERVER_STATUS_IN_TRANS_READONLY  = 8192;
        const SERVER_SESSION_STATE_CHANGED  = 1 << 14;
    }
}