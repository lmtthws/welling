
use headers::http_header_type::*;

#[derive(PartialEq, Debug)]
pub struct HttpHeader {
    field_name: String,
    values: Vec<String>
}

impl HttpHeader {
    pub fn init(field_name: &str) -> HttpHeader {
        HttpHeader {
            field_name: String::from(field_name.to_uppercase()),
            values: Vec::new()
        }
    }

    pub fn get_header_name(&self) -> &str {
        self.field_name.as_ref()
    }

    pub(crate) fn add(&mut self, value: String) {
        if value.len() > 0 {
            self.values.push(value)
        }
    }

    pub(crate) fn get(&self, ix: usize) -> Option<&String> {
        self.values.get(ix)
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn get_all(&self) -> &Vec<String> {
        &self.values
    }

    pub fn parse(self) -> HttpHeaderType {
        let mut header = HttpHeaderType::from_raw_name(self.field_name.as_ref());
        header.parse(self.values);
        header
    }

    //TODO: impl iterator for a wrapper around values indicating quoted/comment/or plain
}



//todo: create case insensitive string wrapper? 

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn init__recognized_string__header_has_expected_type() {
        let header = HttpHeader::init("Host");
        assert_eq!(header.get_header_name(), "HOST")
    }

    #[test]
    fn init__recognized_string__is_case_insensitive() {
        let header = HttpHeader::init("hoSt");
        assert_eq!(header.get_header_name(), "HOST")
    }

    #[test]
    fn init__values_is_empty() {
        let header = HttpHeader::init("abc123");
        assert_eq!(header.len(), 0)
    }

    #[test]
    fn add__arbitrary_value__appended_to_value_list() {
        let mut header = HttpHeader::init("abc123");
        header.add(String::from("vaLue"));
        assert_eq!(header.get(0).unwrap(), "vaLue")
    }

    #[test]
    fn add__multiple_values__list_grows() {
        let mut header = HttpHeader::init("abc123");
        header.add(String::from("vaLue"));
        header.add(String::from("vaLue2"));

        assert_eq!(header.get(0).unwrap(), "vaLue");
        assert_eq!(header.get(1).unwrap(), "vaLue2");
    }

    #[test]
    fn add__empty_value__not_added() {
        let mut header = HttpHeader::init("abc123");
        header.add(String::from(""));

        assert_eq!(header.len(), 0);
    }

    #[test]
    fn parse__recognized_string__header_has_expected_type() {
        let header = HttpHeader::init("Host");
        let header = header.parse();
        assert_eq!(header, HttpHeaderType::Host)
    }

    #[test]
    fn parse__unrecognized_string__uses_extension_header_type() {
        let header = HttpHeader::init("abc123");
        let header = header.parse();
        assert_eq!(header, HttpHeaderType::Extension(String::from("ABC123")))
    }
}

