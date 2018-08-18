use data::DataColumn;
use mysql::data::column::{ColFieldType, ColumnDefinitionFlags};
use mysql::packets::*;

pub enum ResultSetHasMetadata {
    None, //0
    Full //1
}

impl ResultSetHasMetadata {
    pub fn from(flag: u8) -> Result<ResultSetHasMetadata,String> {
        match flag {
            0 => Ok(ResultSetHasMetadata::None),
            1 => Ok(ResultSetHasMetadata::Full),
            _ => Err(String::from("Result set metadata type not recognized"))
        }
    }
}


pub struct ColumnCount {
    metadata_follows: Option<ResultSetHasMetadata>, //omitted if client does not have CLIENT_OPTIONAL_RESULTSET_METADATA capability
    column_count: LengthInteger
}

impl ReadablePacket for ColumnCount {
    fn read<R: Read>(buffer: &mut BufReader<R>, _header: &Header) -> Result<ColumnCount, String> {
       let metadata_follows: Option<ResultSetHasMetadata> = None;
       let column_count = buffer.next_length_integer()?;

        Ok(ColumnCount { metadata_follows, column_count })
    }
}

impl ColumnCount {
    pub fn expected_columns(&self) -> u64 {
        if let Some(ResultSetHasMetadata::None) = self.metadata_follows {
            0
        } else {
            self.column_count.value()
        }
    }
}

#[allow(unused)]
pub struct ColumnDefinition {
    catalog: LengthEncodedString,
    schema: LengthEncodedString,
    table: LengthEncodedString, //virtual table name
    org_table: LengthEncodedString, //physical table name
    name: LengthEncodedString, //virtual col name
    org_name: LengthEncodedString, //physical col name
    length: LengthInteger, //should be 0x0c => 13
    char_set: u16,
    col_length: u32,
    col_type: ColFieldType,
    flags: ColumnDefinitionFlags,
    decimals: u8,
}

impl ReadablePacket for ColumnDefinition {
    fn read<R: Read>(buffer: &mut BufReader<R>, _header: &Header) -> Result<ColumnDefinition, String> {
        let catalog = buffer.next_length_string()?;
        let schema = buffer.next_length_string()?;
        let table = buffer.next_length_string()?;
        let org_table = buffer.next_length_string()?;
        let name = buffer.next_length_string()?;
        let org_name = buffer.next_length_string()?;
        let length = buffer.next_length_integer()?;
        let char_set = buffer.next_u16()?;
        let col_length = buffer.next_u32()?;

        let col_type = buffer.next_u8()?;
        let col_type = ColFieldType::from(col_type)?;

        let flags = buffer.next_u16()?;
        let flags = ColumnDefinitionFlags::from_bits_truncate(flags);

        let decimals = buffer.next_u8()?;

        Ok(ColumnDefinition {
            catalog, schema, table, org_table, name, org_name, length, char_set, col_length, col_type, flags, decimals
        })
    }
}

impl ColumnDefinition {
    pub fn to_data_col(&self) -> DataColumn {
        let col_type = self.col_type.to_data_col(self.flags);
        DataColumn {
            name: String::from(self.name.text()),
            nullable: !self.flags.contains(ColumnDefinitionFlags::NOT_NULL),
            col_type
        }
    }
}
