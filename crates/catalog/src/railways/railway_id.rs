use common::slug::Slug;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct RailwayId(Slug);

impl RailwayId {
    pub fn new(id: &str) -> Self {
        let slug = Slug::new(id);
        RailwayId(slug)
    }
}

impl fmt::Display for RailwayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod railway_ids {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_railway_ids() {
            let railway_id = RailwayId::new("railway name");
            assert_eq!("railway-name", railway_id.to_string());
        }

        #[test]
        fn it_should_compare_two_railway_ids() {
            let id1 = RailwayId::new("railway name");
            let id2 = RailwayId::new("railway name");
            assert_eq!(id1, id2);
        }
    }
}
