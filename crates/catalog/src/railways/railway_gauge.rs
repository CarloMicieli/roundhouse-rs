use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RailwayGauge {
    meters: Decimal,
}

impl RailwayGauge {
    pub fn new(meters: Decimal) -> Self {
        RailwayGauge { meters }
    }

    pub fn meters(&self) -> Decimal {
        self.meters
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_gauges {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_railway_gauges() {
            let gauge = RailwayGauge::new(dec!(1.435));
            assert_eq!(dec!(1.435), gauge.meters());
        }
    }
}
