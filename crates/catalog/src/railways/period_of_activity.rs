use chrono::NaiveDate;

#[derive(Debug)]
pub struct PeriodOfActivity {
    operating_since: Date,
    operating_until: Option<Date>,
    status: RailwayStatus,
}

impl PeriodOfActivity {
    pub fn new(
        operating_since: Date,
        operating_until: Option<Date>,
        status: RailwayStatus,
    ) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until,
            status,
        }
    }

    pub fn active_railway(operating_since: Date) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until: None,
            status: RailwayStatus::Active,
        }
    }

    pub fn inactive_railway(
        operating_since: Date,
        operating_until: Date,
    ) -> Self {
        PeriodOfActivity {
            operating_since,
            operating_until: Some(operating_until),
            status: RailwayStatus::Active,
        }
    }

    pub fn operating_since(&self) -> &Date {
        &self.operating_since
    }

    pub fn operating_until(&self) -> Option<&Date> {
        self.operating_until.as_ref()
    }

    pub fn status(&self) -> RailwayStatus {
        self.status
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Date {
    Year(u8),
    ExactDay(NaiveDate),
}

impl Date {
    pub fn with_year(year: u8) -> Self {
        Date::Year(year)
    }

    pub fn with_exact_day(date: NaiveDate) -> Self {
        Date::ExactDay(date)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RailwayStatus {
    Active,
    Inactive,
}

#[cfg(test)]
mod test {
    use super::*;

    mod periods_of_activity {
        use super::*;
    }
}
