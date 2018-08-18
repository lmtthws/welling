use std::fmt::{Display, Formatter, Error};

pub struct StatusCode {
    code_class: StatusCodeClass, 
    flavor: u32
}

macro_rules! declareCode {
	($n: ident, $c: expr) => { pub fn $n() -> StatusCode {
            StatusCode {
                code_class:  StatusCodeClass::from_code($c),
                flavor: $c % 100
            }
        }
    }
}


impl StatusCode {
    declareCode!(continue_code,100);
    declareCode!(switching_protocols,101);


    fn get_code(&self) -> u32 {
        let base_code = self.code_class.get_base_code();
        base_code + self.flavor
    }

    fn is_cacheable(&self) -> bool {
        match self.get_code() {
            200 | 203 | 204 | 206 |
            300 | 301 |
            404 | 405 | 410 | 414 |
            501 => true,
            _ => false
        }
    }
}


pub enum StatusCodeClass {
    Informational,
    Successful,
    Redirection,
    ClientError,
    ServerError
}

impl StatusCodeClass {
    fn get_base_code(&self) -> u32 {
        match *self {
            StatusCodeClass::Successful => 200,
            StatusCodeClass::Informational => 100,
            StatusCodeClass::Redirection => 300,
            StatusCodeClass::ClientError => 400,
            StatusCodeClass::ServerError => 500
        }
    }

    fn from_code(code: u32) -> StatusCodeClass {
        let base_code = (code / 100) * 100;
        match base_code {
            100 => StatusCodeClass::Informational,
            200 => StatusCodeClass::Successful,
            300 => StatusCodeClass::Redirection,
            400 => StatusCodeClass::ClientError,
            500 => StatusCodeClass::ServerError,
            _ => panic!("Unknown status code") 
        }
    }
}

impl Display for StatusCodeClass {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        write!(f, "{}", self.get_base_code())
    }
}