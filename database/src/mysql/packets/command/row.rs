use mysql::packets::*;
use mysql::packets::bytes::Endian;
use mysql::packets::protocol_reader::ProtocolTypeConverter;

pub enum RawValue {
    Null,
    Valued(LengthEncodedString)
}

const NULL_VALUE: u8 = 0xFB;

impl RawValue {
    fn read<R: Read>(buffer: &mut BufReader<R>) -> Result<RawValue, String> {
        let val = buffer.next_u8()?;
        match val {
            NULL_VALUE => Ok(RawValue::Null),
            _ => { 
                let length: u64;
                match val {
                    0xFF => return Err(format!("Expected fixed length integer, but first byte was invalid ({})", val)),
                    LengthInteger::TWO_BYTE_PREFIX => length = protocol_reader::read_exact(buffer, 2)?.to_u16(Endian::Big) as u64,
                    LengthInteger::THREE_BYTE_PREFIX => length = protocol_reader::read_exact(buffer, 3)?.to_u24(Endian::Big).0 as u64,
                    LengthInteger::EIGHT_BYTE_PREFIX => length = protocol_reader::read_exact(buffer, 8)?.to_u64(Endian::Big),
                    _ => length = val as u64
                }

                let text = buffer.next_fixed_string(length)?;
                let length = LengthInteger::new(length);
                Ok(RawValue::Valued(LengthEncodedString::from_unchecked(length, text)))
            } 
        }
    }

    fn byte_len(&self) -> u32 {
        match *self {
            RawValue::Null => 1,
            RawValue::Valued(ref val) => val.packet_size() as u32 //if it's in a single packet, it must be < 16mb
        }
    }
}


pub struct ResultSetRow {
    col_count: LengthInteger,
    values: Vec<RawValue>,
    terminator: Option<EofPacket41>
}

impl ReadablePacket for ResultSetRow {
    fn read<R: Read>(buffer: &mut BufReader<R>, header: &Header) -> Result<ResultSetRow, String> {
        let mut total_bytes = header.packet_len().0;

        let mut values = Vec::new();
        while total_bytes > 0 {
            let raw_value = RawValue::read(buffer)?;
            total_bytes -= raw_value.byte_len();
            values.push(raw_value);
        }

        //assume client_deprecate_eof capability is set until we refactor into a parser that is aware of capabilitities...
        let col_count = LengthInteger::new(values.len() as u64);
        let terminator = None;
        Ok(ResultSetRow{col_count, values, terminator})
    }
}