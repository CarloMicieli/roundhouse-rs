#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Socials {
    facebook: Option<Handler>,
    instagram: Option<Handler>,
    linkedin: Option<Handler>,
    twitter: Option<Handler>,
    youtube: Option<Handler>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Handler(String);
