use std::fmt;
use std::str;

// The power methods for the model.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PowerMethod {
    /// Direct current.
    DC,

    /// Alternating current (Maerklin).
    AC,
}

impl fmt::Display for PowerMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for PowerMethod {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DC" => Ok(PowerMethod::DC),
            "AC" => Ok(PowerMethod::AC),
            _ => Err("Invalid value for power methods [allowed: 'AC' or 'DC']"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod power_methods {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_parse_string_as_power_methods() {
            let pm = "AC".parse::<PowerMethod>();
            assert!(pm.is_ok());
            assert_eq!("AC", pm.unwrap().to_string());
        }
    }
}
