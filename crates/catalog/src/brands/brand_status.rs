#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BrandStatus {
    Active,
    OutOfBusiness,
}

impl Default for BrandStatus {
    fn default() -> Self {
        BrandStatus::Active
    }
}
