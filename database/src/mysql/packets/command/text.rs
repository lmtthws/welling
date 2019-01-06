use ::std::io::{Read, BufReader, Cursor, Write, BufWriter};
use ::std::u8;

use crate::*;
use crate::mysql::packets::{Header, ReadablePacket};
use crate::mysql::packets::*;
use crate::mysql::packets::command::SupportedCommands;
use crate::mysql::packets::command::column::{ColumnDefinition, ColumnCount};
use crate::mysql::packets::command::row::ResultSetRow;
use crate::mysql::packets::protocol_reader;

pub struct QueryRequest {
    server_command: SupportedCommands,
    text: String
}
impl QueryRequest {
    pub fn new(query: String) -> QueryRequest {
        QueryRequest {
            text: query, 
            server_command: SupportedCommands::COM_QUERY
        }
    }
}

impl WriteablePacket for QueryRequest {
    fn write<W: Write>(&self, writer: &mut BufWriter<W>) -> Result<(),::std::io::Error> {
       write!(writer, "{}", self.server_command)?;
       write!(writer, "{}", self.text)?;
       Ok(())
    }

    fn calculate_header_size(&self) -> Result<u24,String> {
        Ok(u24(1 + self.text.len() as u32))
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
        if header.packet_len().0 > u8::MAX as u32 {
            return Err(format!("Initial response packet from server to query is larger than expected: {}", header.packet_len().0 ))
        }

        let the_packet = protocol_reader::read_exact(buffer, header.packet_len().0)?;
        
        let mut cursor = Cursor::new(the_packet);

        let mut buffer: [u8;1] = [0;1];
        match cursor.read_exact(&mut buffer) {
            Err(e) => Err(format!("{}", e)),
            Ok(()) => {
                let marker = buffer[0];
                cursor.set_position(0);
                let mut buffer = BufReader::new(cursor);
                match marker {
                    0x00 => Ok(QueryResponse::Okay(OkPacket41::read(&mut buffer, header)?)),
                    0xFF => Ok(QueryResponse::Error(ErrPacket41::read(&mut buffer, header)?)),
                    0xFB => Ok(QueryResponse::LocalInfile(LocalInFileResponse::read(&mut buffer, header)?)),
                    _ => Ok(QueryResponse::ResultSet(TextResultSet::read(&mut buffer, header)?))
                }
            }
        }
    }
}


pub struct LocalInFileResponse {
    _file_name: String,
}

impl ReadablePacket for LocalInFileResponse {
    fn read<R: Read>(_buffer: &mut BufReader<R>, _header: &Header) -> Result<LocalInFileResponse, String> {
        Err(String::from("Local infile not supported"))
    }
}


pub struct TextResultSet {
    column_count: ColumnCount,
    column_definitions: Vec<ColumnDefinition>, //present if either CLIENT_OPTIONAL_RESULTSET_METADATA is not set or server sent ResultSetMetadata::Full
    marker: Option<EofPacket41>, //present if not capabilities and CLIENT_DEPRECATE_EOF <- should never be present for us...
    results: Vec<ResultSetRow>,
    terminator: GeneralResponses
}

impl ReadablePacket for TextResultSet {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<TextResultSet, String> {
        let column_count = ColumnCount::read(buffer, header)?;

        let mut column_definitions = Vec::with_capacity(column_count.expected_columns() as usize);
        for _ in 0..column_count.expected_columns() {
            let column = ServerPacket::<ColumnDefinition>::read(buffer)?;
            let column = column.into_payload();
            column_definitions.push(column);
        }

        let marker: Option<EofPacket41> = None;
       
        let terminator: GeneralResponses;
        let mut results = Vec::new();
        loop {
            let result_row = ServerPacket::<ResultSetComponent>::read(buffer)?;

            match result_row.into_payload() {
                ResultSetComponent::Row(result) => results.push(result),
                ResultSetComponent::Terminator(result) => {
                    terminator = result;
                    break;
                }
            }
        }
        
        Ok(TextResultSet{column_count, column_definitions, marker, results, terminator})
    }
}

impl TextResultSet {
    pub fn to_query_result(self) -> Result<QueryResult,String> {
        match self.terminator {
            GeneralResponses::Error(ref e) => Ok(QueryResult::Error(format!("{}", e))),
            GeneralResponses::Okay(ref ok) => {
                if self.results.len() == 0 {
                    Ok(QueryResult::AffectedRows(ok.affected_rows()))
                } else {
                    let col_collection = self.column_definitions.iter().map(|cd| cd.to_data_col()).collect::<Vec<DataColumn>>();
                    
                    let rows = match self.results.into_iter().map(|r| r.to_data_row((&col_collection).into_iter())).collect() {
                        Ok(rs) => rs,
                        Err(e) => return Err(e)
                    };

                    Ok(QueryResult::Rows(DataTable::build(col_collection, rows )))
                }
            }
        }       
    }
}

enum ResultSetComponent {
    Row(ResultSetRow),
    Terminator(GeneralResponses)
}

impl ReadablePacket for ResultSetComponent {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<ResultSetComponent, String> {
        let the_packet = protocol_reader::read_exact(buffer, header.packet_len().0)?;
        
        let mut cursor = Cursor::new(the_packet);

        let mut buffer: [u8;1] = [0;1];
        match cursor.read_exact(&mut buffer) {
            Err(e) => Err(format!("{}", e)),
            Ok(()) => {
                let marker = buffer[0];
                cursor.set_position(0);
                let mut buffer = BufReader::new(cursor);
                match marker {
                    0x00 | 0xFF => Ok(ResultSetComponent::Terminator(GeneralResponses::read(&mut buffer, header)?)),
                    _ => Ok(ResultSetComponent::Row(ResultSetRow::read(&mut buffer, header)?))
                }
            }
        }
    }
}



/*
CREATE TEMPORARY TABLE ins ( id INT );
DROP PROCEDURE IF EXISTS multi;
DELIMITER $$
CREATE PROCEDURE multi() BEGIN
  SELECT 1;
  SELECT 1;
  INSERT INTO ins VALUES (1);
  INSERT INTO ins VALUES (2);
END$$
DELIMITER ;
CALL multi();
DROP TABLE ins;


packet: column count
01 00 00 01 01 <- 1

packet: column definition
17 00 00 02 
    03 64 65 66 <- catalog: def
    00 <- schema
    00 <- table
    00 <- org_name   ..........def...
    01 31 <- name: 1
    00 <- org_name
    0c <- fixed length fields size
    3f 00 <- character set: 63-binary
    01 00 00 00 <- column length: 1
    08 <- type: LongLong
    81 00 <- flags: 129: 128 | 1 -> Not Null and Binary
    00 <- decimals: 0 - integer
   
    00 00    .1..?...........

05 00 00 03 fe 00 00 0a 00 <- EOF for column terminator

02 00 00 04 01 31 <- there's one column, so there's only one length-string in here: length of 1 and value of 1

05 00 00 05 fe 00 00 0a 00    - EOF packet: 5 chars, sequence 5 -> FE for EOF, warnings: 00 00, status_flags: 0a 00 -> 00 0a -> 10 -> 8 | 2 -> MORE RESULTS and AUTO_COMMIT                         ........


01 00 00 01 01  -> column count
17 00 00 02     
03 64 65 66 00 00 00 01 31 00 0c 3f 00 01 00 00 00
08 81 00 00 00 00    
05 00 00 03 fe 00 00 0a    00 

02 00 00 04 01 31 
05 00 00 05 fe 00 00 0a 00 

*/

