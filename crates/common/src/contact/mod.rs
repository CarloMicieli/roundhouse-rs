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

    /// Returns the mail address for this contact info
    pub fn email(&self) -> Option<&MailAddress> {
        self.email.as_ref()
    }

    /// Returns the phone number for this contact info
    pub fn phone(&self) -> Option<&PhoneNumber> {
        self.phone.as_ref()
    }

    /// Returns the website url for this contact info
    pub fn website_url(&self) -> Option<&WebsiteUrl> {
        self.website_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod contacts {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_contact_info() {
            let email = MailAddress::new("mail@mail.com");
            let website_url = WebsiteUrl::new("http://www.website.com");
            let phone = PhoneNumber::new("+15551234");

            let contact_info = ContactInfo::new(
                Some(email.clone()),
                Some(website_url.clone()),
                Some(phone.clone()),
            );

            assert_eq!(Some(&email), contact_info.email());
            assert_eq!(Some(&phone), contact_info.phone());
            assert_eq!(Some(&website_url), contact_info.website_url());
        }
    }
}
