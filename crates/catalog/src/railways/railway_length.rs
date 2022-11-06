use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RailwayLength {
    kilometers: Decimal,
    miles: Decimal,
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_lengths {
        use super::*;
    }
}
