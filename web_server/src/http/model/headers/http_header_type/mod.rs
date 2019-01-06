mod annotations;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum HttpHeaderType {
    Host,
    Connection,
    Upgrade,
    Http2Settings,
    Extension(String)
}

impl HttpHeaderType {
    pub fn from_raw_name(raw_name: &str) -> HttpHeaderType {
        use self::HttpHeaderType::*;

        match raw_name.to_uppercase().as_ref() {
            "HOST" => Host,
            "CONNECTION" => Connection,
            "UPGRADE" => Upgrade,
            _ => Extension(String::from(raw_name))
        }
    }

    pub fn parse(&mut self, _values: Vec<String>) {
        //be sure to check for empty value list or values of only comments
    }
}
