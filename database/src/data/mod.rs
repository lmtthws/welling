mod column;
mod row;

pub use self::column::{DataColType, DataColumn};
pub use self::row::{DataRow, DataCell, DataCellValue};
pub use self::time::*;

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
    #[derive(Clone)]
    pub struct Timestamp {
        pub raw: String,
        pub date: Option<Date>,
        pub time: Option<Time>
    }

    #[derive(Copy,Clone)]
    pub struct Date {
        month: Month,
        day: u8,
        year: u8
    }

    #[derive(Copy,Clone)]
    pub enum Month {
        Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec
    }

    #[derive(Copy,Clone)]
    pub struct Time {
        hour: u8,
        minute: u8,
        second: u8,
        timezone: Timezone
    }

    #[derive(Copy,Clone)]
    pub enum Timezone {
        UTC
    }
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
