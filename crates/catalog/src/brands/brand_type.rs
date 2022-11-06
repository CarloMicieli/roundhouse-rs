use std::str;

/// The different kinds for railway models brands
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BrandType {
    /// These manufactures produce models using the die casting method
    Industrial,

    /// These manufacturers produce models which are made of brass or similar alloys.
    ///
    /// They are usually more expensive than the industrial series due to the limited
    /// production quantities and the "hand made" nature of the production
    BrassModels,
}

impl Default for BrandType {
    fn default() -> Self {
        BrandType::Industrial
    }
}

impl str::FromStr for BrandType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("The brand type value cannot be blank");
        }

        match s {
            "industrial" => Ok(BrandType::Industrial),
            "brass_models" => Ok(BrandType::BrassModels),
            _ => Err("Invalid value for brand type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_types_tests {
        use super::*;

        #[test]
        fn it_should_define_default_brand_type() {
            let brand_type = BrandType::default();
            assert_eq!(BrandType::Industrial, brand_type);
        }
    }
}
