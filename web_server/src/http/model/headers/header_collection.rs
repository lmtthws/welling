use std::collections::HashMap;
use std::collections::hash_map::Values;
use headers::http_header::*;
use headers::http_header_type::*;

pub struct HeaderCollection {
    headers: HashMap<String, HttpHeader>,
}

impl HeaderCollection {
    pub fn init_empty() -> HeaderCollection {
        HeaderCollection {
            headers: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.headers.len()
    }
    
    pub fn add_or_get_mut(&mut self, field_name: &str) -> &mut HttpHeader {
        let header_type = field_name.to_uppercase();
        self.headers.entry(header_type).or_insert(HttpHeader::init(field_name))
    }

    pub fn get(&self, field_name: &str) -> Option<&HttpHeader> {
        self.headers.get(&field_name.to_uppercase())
    }

    pub fn iter(&self) -> Values<'_, String,HttpHeader> {
        self.headers.values()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn get_or_add__new_header__added_to_collection() {
        let mut headers = HeaderCollection::init_empty();

        assert_eq!(headers.len(),0);
        
        {
            let header = headers.add_or_get_mut("Host");
            assert_eq!(header.len(), 0);
            header.add(String::from("testVal"));
        }

        assert_eq!(headers.len(), 1);
       
        {
            let header = (&mut headers).get("Host").unwrap();
            assert_eq!(header.get(0).unwrap(), "testVal");
        }


        assert_eq!(headers.len(), 1);
    }

        #[test]
    fn iter__expected_vals_present() {
        let mut headers = HeaderCollection::init_empty();

        headers.add_or_get_mut("1");
        headers.add_or_get_mut("2");
        headers.add_or_get_mut("3");

        let mut has_one = false;
        let mut has_two = false;
        let mut has_three = false;

        for header in headers.iter() {
            match header.get_header_name() {
                "1" => has_one = true,
                "2" => has_two = true,
                "3" => has_three = true,
                _ => panic!("Unexpected extension header")
            }
        }

        assert!(has_one && has_two && has_three)
    }

}