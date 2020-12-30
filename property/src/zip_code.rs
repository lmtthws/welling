#[derive(PartialEq, Debug)]
pub struct ZipCode {
    area: usize,
    route: Option<usize>
}

impl ZipCode {
    pub fn create(area: usize)-> ZipCode{
        ZipCode{ area, route: None}
    }

    pub fn create_full(area: usize, route: usize) -> ZipCode {
        ZipCode{ area, route: Some(route)}
    }

    pub fn is_full(&self) -> bool {
        self.route.is_none()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn zip_create_area() {
        let z = super::ZipCode::create(19082);
        assert_eq!(z.area, 19082);
        assert_eq!(z.route, None);
    }

    fn zip_create_full() {
        let z = super::ZipCode::create_full(19082, 234);
        assert_eq!(z.area, 19082);
        assert_eq!(z.route, Some(234));
    }

    fn is_full_false_for_partial() {
        let z = super::ZipCode::create(19082);
        assert_eq!(z.is_full(), false);
    }

    fn is_full_true_for_full() {
        let z = super::ZipCode::create_full(19082,1234);
        assert_eq!(z.is_full(), true);
    }
}