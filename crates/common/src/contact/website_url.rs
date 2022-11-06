use std::fmt;
use url::Url;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WebsiteUrl(Url);

impl WebsiteUrl {
    pub fn new(value: &str) -> Result<WebsiteUrl, ()> {
        let url: Url = Url::parse(value).map_err(|_e| ())?;
        Ok(WebsiteUrl(url))
    }
}

impl fmt::Display for WebsiteUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
