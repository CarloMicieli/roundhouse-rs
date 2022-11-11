use crate::catalog_items::catalog_item::Brand;
use crate::catalog_items::item_number::ItemNumber;
use common::slug::Slug;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

/// A unique identifier for a catalog item
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CatalogItemId(Slug);

impl CatalogItemId {
    /// Creates a new catalog item id from its brand and item number
    pub fn new(brand: &Brand, item_number: &ItemNumber) -> Self {
        let value = Slug::of(brand.name(), item_number.value());
        CatalogItemId(value)
    }

    /// Returns the value for this catalog item id
    pub fn value(&self) -> &str {
        self.0.deref()
    }
}

impl Display for CatalogItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for CatalogItemId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(())
        } else {
            let value = Slug::new(s);
            Ok(CatalogItemId(value))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod catalog_item_ids {
        use super::*;
        use crate::brands::brand_id::BrandId;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_catalog_item_ids() {
            let brand = Brand::new(BrandId::new("ACME"), "ACME");
            let item_number = ItemNumber::new("60000").unwrap();
            let id = CatalogItemId::new(&brand, &item_number);

            assert_eq!("acme-60000", id.value());
        }

        #[test]
        fn it_should_display_catalog_item_ids() {
            let id: CatalogItemId =
                CatalogItemId::from_str("acme-60000").unwrap();
            assert_eq!("acme-60000", id.to_string());
        }
    }
}
