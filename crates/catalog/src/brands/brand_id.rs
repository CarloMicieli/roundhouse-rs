use common::slug::Slug;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Default)]
pub struct BrandId(Slug);

impl BrandId {
    pub fn new(id: &str) -> Self {
        let slug = Slug::new(id);
        BrandId(slug)
    }
}

impl fmt::Display for BrandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brand_ids {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_brand_ids() {
            let brand_id = BrandId::new("brand name");
            assert_eq!("brand-name", brand_id.to_string());
        }
    }
}
