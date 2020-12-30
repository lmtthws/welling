use crate::zip_code::ZipCode;

pub struct Address {
    street: String,
    city: String,
    country: Country,
    zip_code: ZipCode
}

impl Address {
    pub fn create(street: &str, city: &str, country: Country, zip: usize) -> Address {
        Address{
            street: street.to_string(),
            city: city.to_string(),
            country,
            zip_code: ZipCode::create(zip)
        }
    }

}

#[derive(PartialEq, Debug)]
pub enum Country {
    USA
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_populates_fields() {
        let a = super::Address::create("7115 Hilltop Rd", "Upper Darby", super::Country::USA, 19082);
        assert_eq!(a.street, String::from("7115 Hilltop Rd"));
        assert_eq!(a.city, String::from("Upper Darby"));
        assert_eq!(a.country, super::Country::USA);
        assert_eq!(a.zip_code, ::zip_code::ZipCode::create(19082));
    }
}