use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(phone_number: &str) -> Self {
        PhoneNumber(String::from(phone_number))
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod phone_numbers {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_phone_numbers() {
            let phone_number = PhoneNumber::new("555-123456");
            assert_eq!("555-123456", phone_number.to_string());
        }
    }
}
