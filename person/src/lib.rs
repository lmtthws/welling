


pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: usize,
}

impl Person {
    pub fn create(first_name: &str, last_name: &str, age: usize) -> Person {
        Person{ 
            first_name: first_name.to_string(), 
            last_name: last_name.to_string(), 
            age
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_create() {
        let p = ::Person::create("Luke", "Matthews", 32);
        assert_eq!(p.full_name(), String::from("Luke Matthews"));
        assert_eq!(p.age, 32);
    }
}