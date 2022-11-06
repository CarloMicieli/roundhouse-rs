pub mod mail_address;
pub mod phone_number;
pub mod website_url;

use crate::contact::mail_address::MailAddress;
use crate::contact::phone_number::PhoneNumber;
use crate::contact::website_url::WebsiteUrl;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContactInfo {
    email: Option<MailAddress>,
    website_url: Option<WebsiteUrl>,
    phone: Option<PhoneNumber>,
}

impl ContactInfo {
    pub fn new(
        email: Option<MailAddress>,
        website_url: Option<WebsiteUrl>,
        phone: Option<PhoneNumber>,
    ) -> Self {
        ContactInfo {
            email,
            website_url,
            phone,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod contacts {
        use super::*;
        use pretty_assertions::assert_eq;
    }
}
