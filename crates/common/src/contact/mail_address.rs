use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MailAddress(String);

impl MailAddress {
    pub fn new(mail_address: &str) -> Self {
        MailAddress(String::from(mail_address))
    }
}

impl fmt::Display for MailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod mail_addresses {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_mail_addresses() {
            let mail_address = MailAddress::new("mail@mail.com");
            assert_eq!("mail@mail.com", mail_address.to_string());
        }
    }
}
