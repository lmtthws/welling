mod column;
mod row;
mod types;

pub use self::column::{DataColType, DataColumn};
pub use self::row::{DataRow, DataCell, DataCellValue};
pub use self::types::*;

use ::std::str::FromStr;

pub struct DataTable {
    columns: Vec<DataColumn>,
    rows: Vec<DataRow>
}

impl DataTable {
    pub fn build(columns: Vec<DataColumn>, rows: Vec<DataRow>) -> DataTable {
        DataTable {columns, rows}
    }
}

pub trait CellReader {
    fn to_data_cell(&self, defn: &DataColumn) -> Result<DataCell,String>;
}


mod time {

}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Timestamp{
            raw: String::from(s),
            date: None,
            time: None
        })
    }
}
