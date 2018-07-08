use mysql::packets::{Header, WriteablePacket, ReadablePacket};
use ::std::io::{Read, Write, BufReader, BufWriter};
use mysql::packets::command::SupportedCommands;
use mysql::packets::protocol_types::*;
use mysql::packets::command::column::ColumnDefinition;
use mysql::packets::command::row::ResultSetRow;
use mysql::packets::general_response::{EofPacket41,ErrPacket41,OkPacket41,GeneralResponses};

pub struct Request {
    header: Header,
    server_command: SupportedCommands,
    text: String
}

pub enum ResultSetMetadata
{
    None, //0
    Full //1
}

impl ResultSetMetadata{
    pub fn from(flag: u8) -> Result<ResultSetMetadata,String> {
        match flag {
            0 => Ok(ResultSetMetadata::None),
            1 => Ok(ResultSetMetadata::Full),
            _ => Err(String::from("Result set metadata type not recognized"))
        }
    }
}

const NULL_VALUE: u8 = 0xFB;

pub struct Response {
    header: Header,
    inner_packet: QueryResponse
}

pub enum QueryResponse {
    LocalInfile(LocalInFileResponse), //0xFB
    ResultSet(TextResultSet) //0xFF
}

pub struct LocalInFileResponse {
    identifier: u8,
    file_name: String,
}

pub struct TextResultSet {
    metadata_follows: Option<ResultSetMetadata>,
    col_count: LengthInteger,
    column_definitions: Vec<ColumnDefinition>, //present if either CLIENT_OPTIONAL_RESULTSET_METADATA is not set or server sent ResultSetMetadata::Full
    marker: Option<EofPacket41>, //present if not capabilities and CLIENT_DEPRECATE_EOF <- should never be present for us...
    results: Vec<ResultSetRow>,
    terminator: GeneralResponses
}



