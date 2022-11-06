use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Metadata {
    version: u8,
    created_at: DateTime<Utc>,
    last_modified: Option<DateTime<Utc>>,
}

impl Metadata {
    pub fn new(
        version: u8,
        created_at: DateTime<Utc>,
        last_modified: Option<DateTime<Utc>>,
    ) -> Self {
        Metadata {
            version,
            created_at,
            last_modified,
        }
    }

    pub fn created_at(created_at: DateTime<Utc>) -> Self {
        Metadata {
            version: 1u8,
            created_at,
            last_modified: None,
        }
    }

    pub fn updated_at(self, last_modified: DateTime<Utc>) -> Self {
        Metadata {
            version: self.version + 1,
            created_at: self.created_at,
            last_modified: Some(last_modified),
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Metadata::created_at(now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod metadata {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_metadata() {
            let now: DateTime<Utc> = Utc::now();
            let metadata = Metadata::created_at(now);
            assert_eq!(1, metadata.version);
            assert_eq!(now, metadata.created_at);
            assert_eq!(None, metadata.last_modified);
        }

        #[test]
        fn it_should_update_metadata() {
            let now: DateTime<Utc> = Utc::now();
            let metadata = Metadata::created_at(now).updated_at(now);
            assert_eq!(2, metadata.version);
            assert_eq!(now, metadata.created_at);
            assert_eq!(Some(now), metadata.last_modified);
        }
    }
}
