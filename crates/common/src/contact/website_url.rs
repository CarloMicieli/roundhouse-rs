use std::fmt;
use url::Url;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WebsiteUrl(Url);

impl WebsiteUrl {
    /// Create a new website url
    pub fn new(value: &str) -> WebsiteUrl {
        let url: Url = Url::parse(value).expect("invalid url");
        WebsiteUrl(url)
    }
}

impl fmt::Display for WebsiteUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for WebsiteUrl {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let url = Url::parse(value).map_err(|_e| ())?;
        Ok(WebsiteUrl(url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod website_urls {
        use super::*;

        #[test]
        fn it_should_convert_str_to_website_urls() {
            let result: Result<WebsiteUrl, ()> =
                "http://www.website.com".try_into();
            assert!(result.is_ok());
            assert_eq!("http://www.website.com/", result.unwrap().to_string());
        }
    }
}
