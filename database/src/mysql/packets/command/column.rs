use mysql::data::column::{ColFieldType, ColumnDefinitionFlags};
use mysql::packets::*;

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

impl ColumnDefinition {
    pub fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<ColumnDefinition, String> {
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

        Ok(ColumnDefinition{
            catalog, schema, table, org_table, name, org_name, length, char_set, col_length, col_type, flags, decimals
        })
    }
}

