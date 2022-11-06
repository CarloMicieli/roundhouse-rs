use super::brand_id::BrandId;
use super::brand_status::BrandStatus;
use super::brand_type::BrandType;
use common::address::Address;
use common::contact::ContactInfo;
use common::metadata::Metadata;
use common::socials::Socials;
use std::fmt;

/// A model railways manufacturer.
#[derive(Debug, Default)]
pub struct Brand {
    brand_id: BrandId,
    name: String,
    registered_company_name: Option<String>,
    group_name: Option<String>,
    description: Option<String>,
    address: Option<Address>,
    contact_info: Option<ContactInfo>,
    brand_type: BrandType,
    status: BrandStatus,
    socials: Option<Socials>,
    metadata: Metadata,
}

impl Brand {
    pub fn new(
        brand_id: BrandId,
        name: &str,
        registered_company_name: Option<&str>,
        group_name: Option<&str>,
        description: Option<&str>,
        brand_type: BrandType,
        status: BrandStatus,
        address: Option<Address>,
        contact_info: Option<ContactInfo>,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Brand {
            brand_id,
            name: String::from(name),
            registered_company_name: registered_company_name.map(|s| String::from(s)),
            group_name: group_name.map(|s| String::from(s)),
            description: description.map(|s| String::from(s)),
            address,
            contact_info,
            brand_type,
            status,
            socials,
            metadata,
        }
    }

    /// Returns this brand unique identifier
    pub fn brand_id(&self) -> &BrandId {
        &self.brand_id
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this brand description
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Returns this brand registered company name
    pub fn registered_company_name(&self) -> Option<&String> {
        self.registered_company_name.as_ref()
    }

    pub fn group_name(&self) -> Option<&String> {
        self.group_name.as_ref()
    }

    /// Returns this brand type
    pub fn brand_type(&self) -> BrandType {
        self.brand_type
    }

    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.status == BrandStatus::Active
    }

    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;
        use crate::brands::brand::tests::data::acme;
        use chrono::{DateTime, Utc};
        use common::contact::{mail_address::MailAddress, website_url::WebsiteUrl};
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_brands() {
            let now: DateTime<Utc> = Utc::now();
            let contact_info = ContactInfo::new(
                Some(MailAddress::new("mail@acmetreni.com")),
                Some(WebsiteUrl::new("http://www.acmetreni.com").unwrap()),
                None,
            );

            let address = Address::builder()
                .street_address("Viale Lombardia, 27")
                .postal_code("20131")
                .city("Milano")
                .region("MI")
                .country_code(CountryCode::ITA)
                .build()
                .unwrap();

            let brand = Brand::new(
                BrandId::new("ACME"),
                "ACME",
                Some("Associazione Costruzioni Modellistiche Esatte"),
                None,
                None,
                BrandType::Industrial,
                BrandStatus::Active,
                Some(address.clone()),
                Some(contact_info.clone()),
                None,
                Metadata::created_at(now),
            );

            assert_eq!(BrandId::new("ACME"), brand.brand_id);
            assert_eq!("ACME", brand.name);
            assert_eq!(
                Some("Associazione Costruzioni Modellistiche Esatte".to_string()),
                brand.registered_company_name
            );
            assert_eq!(None, brand.group_name);
            assert_eq!(None, brand.description);
            assert_eq!(BrandType::Industrial, brand.brand_type);
            assert_eq!(BrandStatus::Active, brand.status);
            assert_eq!(Some(address), brand.address);
            assert_eq!(None, brand.socials);
            assert_eq!(Metadata::created_at(now), brand.metadata);
        }

        #[test]
        fn is_should_display_brands() {
            let acme = acme();
            assert_eq!("ACME", acme.to_string());
        }
    }

    mod data {
        use crate::brands::brand::Brand;
        use crate::brands::brand_id::BrandId;
        use crate::brands::brand_status::BrandStatus;
        use crate::brands::brand_type::BrandType;
        use chrono::{DateTime, Utc};
        use common::address::Address;
        use common::contact::{mail_address::MailAddress, website_url::WebsiteUrl, ContactInfo};
        use common::metadata::Metadata;
        use isocountry::CountryCode;

        pub fn acme() -> Brand {
            let now: DateTime<Utc> = Utc::now();
            let contact_info = ContactInfo::new(
                Some(MailAddress::new("mail@acmetreni.com")),
                Some(WebsiteUrl::new("http://www.acmetreni.com").unwrap()),
                None,
            );

            let address = Address::builder()
                .street_address("Viale Lombardia, 27")
                .postal_code("20131")
                .city("Milano")
                .region("MI")
                .country_code(CountryCode::ITA)
                .build()
                .unwrap();

            Brand::new(
                BrandId::new("ACME"),
                "ACME",
                Some("Associazione Costruzioni Modellistiche Esatte"),
                None,
                None,
                BrandType::Industrial,
                BrandStatus::Active,
                Some(address.clone()),
                Some(contact_info.clone()),
                None,
                Metadata::created_at(now),
            )
        }
    }
}
