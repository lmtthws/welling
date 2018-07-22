use data::Timestamp;

pub struct DataRow {
    pub values: Vec<DataCell>
}

impl DataRow {
    pub fn new(values: Vec<DataCell>) -> DataRow {
        DataRow{values}
    }
}


pub enum DataCellValue {
    SignedInteger(i64),
    UnsignedInteger(u64),
    Float(f64),
    Timestamp(Timestamp),
    Bool(bool),
    VarChar(String)
}


pub struct DataCell {
    value: Option<DataCellValue>
}

impl DataCell {
    pub fn new(value: Option<DataCellValue>) -> DataCell {
        DataCell{value}
    }

    pub fn value(&self) -> Option<&DataCellValue> {
        match self.value {
            None => None,
            Some(ref v) => Some(v)
        }
    }
}
