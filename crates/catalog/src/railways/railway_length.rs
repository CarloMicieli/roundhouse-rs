use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RailwayLength {
    kilometers: Decimal,
    miles: Decimal,
}

impl RailwayLength {
    pub fn new(kilometers: Decimal, miles: Decimal) -> Self {
        RailwayLength { kilometers, miles }
    }

    pub fn kilometers(&self) -> Decimal {
        self.kilometers
    }

    pub fn miles(&self) -> Decimal {
        self.miles
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railway_lengths() {
            let miles = dec!(100);
            let kilometers = dec!(100);
            let len = RailwayLength::new(kilometers, miles);
            assert_eq!(miles, len.miles());
            assert_eq!(kilometers, len.kilometers());
        }
    }
}
