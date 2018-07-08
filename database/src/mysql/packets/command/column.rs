use mysql::packets::Header;
use mysql::data::column::{ColFieldType, ColumnDefinitionFlags};
use mysql::packets::protocol_types::*;

pub struct ColumnDefinition {
    header: Header,
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