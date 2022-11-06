use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RailwayGauge {
    meters: Decimal,
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_gauges {
        use super::*;
    }
}
