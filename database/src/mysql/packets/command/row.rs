use mysql::packets::protocol_types::*;
use mysql::packets::general_response::EofPacket41;

pub enum RawValue {
    Null,
    Valued(LengthEncodedString)
}

pub struct ResultSetRow {
    col_count: LengthInteger,
    values: Vec<RawValue>,
    terminator: Option<EofPacket41>
}