use mysql::packets::{Header, WriteablePacket, ReadablePacket};
use ::std::io::{Read, Write, BufReader, BufWriter};
use mysql::packets::protocol_reader::ProtocolTypeReader;
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


pub enum QueryResponse {
    Error(ErrPacket41),
    Okay(OkPacket41),
    LocalInfile(LocalInFileResponse), //0xFB
    ResultSet(TextResultSet) //0xFF
}

impl ReadablePacket for QueryResponse {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<QueryResponse, String> {
        match buffer.next_u8()? {
            0x00 => Ok(QueryResponse::Okay(OkPacket41::read(buffer, header)?)),
            0xFF => Ok(QueryResponse::Error(ErrPacket41::read(buffer, header)?)),
            0xFB => Ok(QueryResponse::LocalInfile(LocalInFileResponse::read(buffer, header)?)),
            _ => Ok(QueryResponse::ResultSet(TextResultSet::read(buffer, header)?))
        }
    }
}



pub struct LocalInFileResponse {
    file_name: String,
}

impl ReadablePacket for LocalInFileResponse {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<LocalInFileResponse, String> {
        Err(String::from("Local infile not supported"))
    }
}



pub struct TextResultSet {
    metadata_follows: Option<ResultSetMetadata>,
    col_count: LengthInteger,
    column_definitions: Vec<ColumnDefinition>, //present if either CLIENT_OPTIONAL_RESULTSET_METADATA is not set or server sent ResultSetMetadata::Full
    marker: Option<EofPacket41>, //present if not capabilities and CLIENT_DEPRECATE_EOF <- should never be present for us...
    results: Vec<ResultSetRow>,
    terminator: GeneralResponses
}

impl ReadablePacket for TextResultSet {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<TextResultSet, String> {
               
        let total_set_length = buffer.next_length_integer()?;

        let mut metadata_follows = None;
        let mut col_count = buffer.next_length_integer()?;
        if let LengthInteger::U8(flag) = col_count {
            if let Ok(metadata_flag) = ResultSetMetadata::from(flag) {
                col_count = buffer.next_length_integer()?;
                metadata_follows = Some(metadata_flag);
            }
        }


        ///TODO: add packets
        let mut column_definitions = Vec::with_capacity(col_count.value() as usize);
        match metadata_follows {
            Some(ResultSetMetadata::None) => (),
            _ => {
                for i in 0..col_count.value() {
                    column_definitions.push(ColumnDefinition::read(buffer)?)
                }
            }
        }

        //assume deprecate_eof is set
        let marker: Option<EofPacket41> = None;

        //TODO: is the result set an actual packet? Otherwise, how do I know how many rows there are????

        Err(String::from("error"))
        
    }
}




