use mysql::packets::protocol_types::*;


pub enum ColFieldType {
    Decimal,
    Tiny,
    Short,
    Long,
    Float,
    Double,
    Null,
    Timestamp,
    LongLong,
    Int24,
    Date,
    Time,
    DateTime,
    Year,
    NewDate,
    VarChar,
    Bit,
    Timestamp2,
    DateTime2,
    Time2,
    Json, //245
    NewDeciaml, //246
    Enum, //247
    Set, //248
    TinyBlob, //249
    MediumBlob, //250
    LongBlob, //251
    Blob, //252
    VarString, //253
    MySqlString, //254,
    Geometry, //255
}

impl ColFieldType {
    pub fn from(field_type: u8) -> Result<ColFieldType,String> {
        match field_type {
            0 => Ok(ColFieldType::Decimal),
            1 => Ok(ColFieldType::Tiny),
            2 => Ok(ColFieldType::Short),
            3 => Ok(ColFieldType::Long),
            4 => Ok(ColFieldType::Float),
            5 => Ok(ColFieldType::Double),
            6 => Ok(ColFieldType::Null),
            7 => Ok(ColFieldType::Timestamp),
            8 => Ok(ColFieldType::LongLong),
            9 => Ok(ColFieldType::Int24),
            10 => Ok(ColFieldType::Date),
            11 => Ok(ColFieldType::Time),
            12 => Ok(ColFieldType::DateTime),
            13 => Ok(ColFieldType::Year),
            14 => Ok(ColFieldType::NewDate),
            15 => Ok(ColFieldType::VarChar),
            16 => Ok(ColFieldType::Bit),
            17 => Ok(ColFieldType::Timestamp2),
            18 => Ok(ColFieldType::DateTime2),
            19 => Ok(ColFieldType::Time2),
            245 => Ok(ColFieldType::Json), //245
            246 => Ok(ColFieldType::NewDeciaml), //246
            247 => Ok(ColFieldType::Enum), //247
            248 => Ok(ColFieldType::Set), //248
            249 => Ok(ColFieldType::TinyBlob), //249
            250 => Ok(ColFieldType::MediumBlob), //250
            251 => Ok(ColFieldType::LongBlob), //251
            252 => Ok(ColFieldType::Blob), //252
            253 => Ok(ColFieldType::VarString), //253
            254 => Ok(ColFieldType::MySqlString), //254,
            255 => Ok(ColFieldType::Geometry), //255
            _ => Err(String::from("MySQL column type not recognized"))
        }
    }
}

//https://dev.mysql.com/doc/dev/mysql-server/8.0.0/mysql__com_8h_source.html
bitflags! {
    pub struct ColumnDefinitionFlags: u16 {
        const NOT_NULL = 1;
        const PRI_KEY = 2;
        const UNIQUE_KEY = 4;
        const MULTIPLE_KEY = 8;
        const BLOB = 16;
        const UNSIGNED = 32;
        const ZEROFILL = 64;
        const BINARY = 128;
        const ENUM = 256;
        const AUTO_INCREMENT = 512;
        const TIMESTAMP = 1024;
        const SET = 2048;
        const NO_DEFAULT_VALUE = 4096;
        const ON_UPDATE_NOW = 8192;
        const NUM = 32768;
        //others are marked as internal
    }
}


