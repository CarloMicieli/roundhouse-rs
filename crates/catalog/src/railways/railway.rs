use crate::railways::railway_id::RailwayId;
use common::contact::ContactInfo;
use common::metadata::Metadata;
use common::socials::Socials;
use isocountry::CountryCode;

use crate::railways::period_of_activity::PeriodOfActivity;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_length::RailwayLength;
use crate::railways::railway_ownership::Ownership;
use std::fmt;
use std::fmt::Formatter;

/// A railway company is a company within the rail industry.
///
/// It can be a manufacturing firm or an operator. Some railway companies operate both the trains
/// and the track, while, particularly in the European Union, operation of the track is undertaken
/// by infrastructure operators and trains are run by different companies.
///
/// Railway companies can be private or public.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Railway {
    railway_id: RailwayId,
    name: String,
    registered_company_name: String,
    description: Option<String>,
    period_of_activity: Option<PeriodOfActivity>,
    length: Option<RailwayLength>,
    gauge: Option<RailwayGauge>,
    country: CountryCode,
    ownership: Option<Ownership>,
    headquarters: Option<String>,
    contact_info: Option<ContactInfo>,
    socials: Option<Socials>,
    metadata: Metadata,
}

impl Railway {
    /// Create new railway
    pub fn new(
        railway_id: RailwayId,
        name: &str,
        registered_company_name: &str,
        description: Option<&str>,
        period_of_activity: Option<PeriodOfActivity>,
        length: Option<RailwayLength>,
        gauge: Option<RailwayGauge>,
        country: CountryCode,
        ownership: Option<Ownership>,
        headquarters: Option<&str>,
        contact_info: Option<ContactInfo>,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Railway {
            railway_id,
            name: String::from(name),
            registered_company_name: String::from(registered_company_name),
            description: description.map(str::to_string),
            period_of_activity,
            length,
            gauge,
            country,
            ownership,
            headquarters: headquarters.map(str::to_string),
            contact_info,
            socials,
            metadata,
        }
    }

    /// The unique identifier for this Railway company
    pub fn railway_id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// The name for this Railway company
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The registered company name (the more formal denomination)
    /// for this Railway company
    pub fn registered_company_name(&self) -> &str {
        &self.registered_company_name
    }

    /// The description for this Railway company
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// The period of activity (active/inactive) for this Railway company
    pub fn period_of_activity(&self) -> Option<&PeriodOfActivity> {
        self.period_of_activity.as_ref()
    }

    /// Returns the total railway network length controlled by this Railway company
    pub fn length(&self) -> Option<&RailwayLength> {
        self.length.as_ref()
    }

    pub fn gauge(&self) -> Option<&RailwayGauge> {
        self.gauge.as_ref()
    }

    pub fn country(&self) -> CountryCode {
        self.country
    }

    pub fn headquarters(&self) -> Option<&str> {
        self.headquarters.as_deref()
    }

    pub fn ownership(&self) -> Option<Ownership> {
        self.ownership
    }

    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// Returns the metadata for this Railway company
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Railway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", &self.name, self.registered_company_name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railways {
        use super::*;
        use chrono::Utc;
        use common::contact::website_url::WebsiteUrl;
        use common::socials::SocialsBuilder;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railways() {
            let metadata = Metadata::created_at(Utc::now());
            let socials = SocialsBuilder::default()
                .instagram("fsitaliane".try_into().ok())
                .linkedin("ferrovie-dello-stato-s-p-a-".try_into().ok())
                .twitter("FSitaliane".try_into().ok())
                .youtube("fsitaliane".try_into().ok())
                .build()
                .ok();
            let length = RailwayLength::of_kilometers(dec!(24564.0));
            let gauge = RailwayGauge::standard();
            let contact_info = ContactInfo::new(
                None,
                Some(WebsiteUrl::new("https://www.fsitaliane.it")),
                None,
            );

            let railway = Railway::new(
                RailwayId::new("FS"),
                "FS",
                "Ferrovie dello stato italiane",
                None,
                None,
                Some(length),
                Some(gauge.clone()),
                CountryCode::ITA,
                Some(Ownership::Public),
                Some("Rome"),
                Some(contact_info.clone()),
                socials.clone(),
                metadata.clone(),
            );

            assert_eq!(&RailwayId::new("FS"), railway.railway_id());
            assert_eq!("FS", railway.name());
            assert_eq!(
                "Ferrovie dello stato italiane",
                railway.registered_company_name()
            );
            assert_eq!(Some("Rome"), railway.headquarters());
            assert_eq!(Some(&length), railway.length());
            assert_eq!(Some(&gauge), railway.gauge());
            assert_eq!(Some(Ownership::Public), railway.ownership());
            assert_eq!(Some(&contact_info), railway.contact_info());
            assert_eq!(socials.as_ref(), railway.socials());
            assert_eq!(&metadata, railway.metadata());
        }

        #[test]
        fn it_should_display_railways() {
            let metadata = Metadata::created_at(Utc::now());
            let railway = Railway::new(
                RailwayId::new("FS"),
                "FS",
                "Ferrovie dello stato italiane",
                None,
                None,
                None,
                None,
                CountryCode::ITA,
                Some(Ownership::Public),
                Some("Rome"),
                None,
                None,
                metadata,
            );

            assert_eq!(
                "FS - Ferrovie dello stato italiane",
                railway.to_string()
            );
        }
    }
}
