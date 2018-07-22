
use data::Timestamp;

/*
pub enum DataColType {
    SignedInt(i64),
    UnsignedInt(u64),
    Float(f64),
    Timestamp(Timestamp),
    Bool(bool),
    VarChar(String)
}*/
pub enum DataColType {
    SignedInt,
    UnsignedInt,
    Float,
    Timestamp,
    Bool,
    VarChar
}

pub struct DataColumn {
   pub name: String,
   pub nullable: bool,
   pub col_type: DataColType,
}

impl DataColumn {

    pub fn col_type(&self) -> &DataColType {
        &self.col_type
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

