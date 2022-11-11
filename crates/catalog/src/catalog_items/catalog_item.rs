use crate::brands::brand_id::BrandId;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::category::Category;
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock::RollingStock;
use crate::scales::scale_id::ScaleId;
use common::metadata::Metadata;
use std::cmp;
use std::fmt;
use std::fmt::Formatter;

/// A catalog item, it can contain one or more rolling stock.
///
/// A catalog item is identified by its catalog item number.
#[derive(Debug, Clone)]
pub struct CatalogItem {
    catalog_item_id: CatalogItemId,
    brand: Brand,
    item_number: ItemNumber,
    category: Category,
    description: Option<String>,
    details: Option<String>,
    scale: Scale,
    power_method: PowerMethod,
    rolling_stocks: Vec<RollingStock>,
    delivery_date: Option<DeliveryDate>,
    count: u8,
    metadata: Metadata,
}

impl PartialEq for CatalogItem {
    fn eq(&self, other: &Self) -> bool {
        self.brand == other.brand && self.item_number == other.item_number
    }
}

impl Eq for CatalogItem {}

impl Ord for CatalogItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let cmp1 = self.brand().cmp(&other.brand());
        if cmp1 == cmp::Ordering::Equal {
            return self.item_number.cmp(&other.item_number);
        }

        cmp1
    }
}

impl PartialOrd for CatalogItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CatalogItem {
    /// Creates a new catalog item
    pub fn new(
        catalog_item_id: CatalogItemId,
        brand: Brand,
        item_number: ItemNumber,
        category: Category,
        description: Option<&str>,
        details: Option<&str>,
        rolling_stocks: Vec<RollingStock>,
        power_method: PowerMethod,
        scale: Scale,
        delivery_date: Option<DeliveryDate>,
        count: u8,
        metadata: Metadata,
    ) -> Self {
        CatalogItem {
            catalog_item_id,
            brand,
            item_number,
            category,
            description: description.map(str::to_string),
            details: details.map(str::to_string),
            scale,
            power_method,
            rolling_stocks,
            delivery_date,
            count,
            metadata,
        }
    }

    pub fn id(&self) -> &CatalogItemId {
        &self.catalog_item_id
    }

    /// Return the Brand for this catalog item.
    pub fn brand(&self) -> &Brand {
        &self.brand
    }

    /// Return the item number as in the corresponding brand catalog.
    pub fn item_number(&self) -> &ItemNumber {
        &self.item_number
    }

    pub fn rolling_stocks(&self) -> &Vec<RollingStock> {
        &self.rolling_stocks
    }

    pub fn is_locomotive(&self) -> bool {
        self.category() == Category::Locomotives
    }

    pub fn category(&self) -> Category {
        self.category
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }

    pub fn scale(&self) -> &Scale {
        &self.scale
    }

    pub fn power_method(&self) -> PowerMethod {
        self.power_method
    }

    pub fn delivery_date(&self) -> &Option<DeliveryDate> {
        &self.delivery_date
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// A model railways manufacturer.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Brand {
    brand_id: BrandId,
    name: String,
}

impl Brand {
    /// Creates a new brand with the given name.
    pub fn new(brand_id: BrandId, name: &str) -> Self {
        Brand {
            brand_id,
            name: name.to_owned(),
        }
    }

    /// Returns this brand unique identifier
    pub fn id(&self) -> &BrandId {
        &self.brand_id
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Brand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Scale {
    scale_id: ScaleId,
    name: String,
}

impl Scale {
    /// Creates a new Scale with the given name.
    pub fn new(scale_id: ScaleId, name: &str) -> Self {
        Scale {
            scale_id,
            name: name.to_owned(),
        }
    }

    /// Returns this brand unique identifier
    pub fn id(&self) -> &ScaleId {
        &self.scale_id
    }

    /// Returns this brand name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod brands {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_brands() {
            let b = Brand::new(BrandId::new("ACME"), "ACME");
            assert_eq!(&BrandId::new("ACME"), b.id());
            assert_eq!("ACME", b.name());
        }

        #[test]
        fn it_should_display_brand_as_string() {
            let b = Brand::new(BrandId::new("ACME"), "ACME");
            assert_eq!("ACME", b.to_string());
        }
    }

    mod scales {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_scales() {
            let s = Scale::new(ScaleId::new("H0"), "H0");
            assert_eq!(&ScaleId::new("H0"), s.id());
            assert_eq!("H0", s.name());
        }

        #[test]
        fn it_should_display_scale_as_string() {
            let s = Scale::new(ScaleId::new("H0"), "H0");
            assert_eq!("H0", s.to_string());
        }
    }
}
