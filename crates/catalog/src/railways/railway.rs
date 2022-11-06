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
#[derive(Debug)]
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

    pub fn ownership(&self) -> Option<&Ownership> {
        self.ownership.as_ref()
    }

    pub fn contact_info(&self) -> Option<&ContactInfo> {
        self.contact_info.as_ref()
    }

    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// The metadata for this Railway company
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Railway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railways {
        use super::*;
    }
}
