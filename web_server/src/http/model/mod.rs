pub use self::request::*;
pub use self::response::*;

pub(crate) mod request;
pub(crate) mod response;

//use std::borrow::Borrow;
use std::iter::Iterator;
use std::collections::HashMap;
use std::collections::hash_map::Values;

pub struct HeaderCollection {
    headers: HashMap<HttpHeaderType, HttpHeader>,
}

impl HeaderCollection {
    pub fn init_empty() -> HeaderCollection {
        HeaderCollection {
            headers: HashMap::new(),
        }
    }
    
    pub fn get_or_add(&mut self, field_name: &str) -> &mut HttpHeader {
        let header_type = HttpHeaderType::from_raw_name(field_name);

        self.headers.entry(header_type).or_insert(HttpHeader::init(field_name))
    }

    pub fn iter(&self) -> Values<'_, HttpHeaderType,HttpHeader> {
        self.headers.values()
    }
}


pub struct HttpHeader {
    header_type: HttpHeaderType,
    values: Vec<String>
}

impl HttpHeader {
    pub fn init(field_name: &str) -> HttpHeader {
        HttpHeader {
            header_type: HttpHeaderType::from_raw_name(&field_name),
            values: Vec::new()
        }
    }

    pub(crate) fn add(&mut self, value: String) {
        self.values.push(value);
    }
}
//todo: create case insensitive string wrapper? 


#[derive(PartialEq, Eq, Hash)]
pub enum HttpHeaderType {
    Host,
    Connection,
    Upgrade,
    Http2Settings,
    Extension(String)
}


impl HttpHeaderType {
    pub fn from_raw_name(raw_name: &str) -> HttpHeaderType {
        use HttpHeaderType::*;

        match raw_name.to_uppercase().as_ref() {
            "HOST" => Host,
            "CONNECTION" => Connection,
            "UPGRADE" => Upgrade,
            _ => Extension(String::from(raw_name))
        }
    }
}