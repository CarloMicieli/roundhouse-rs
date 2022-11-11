#[derive(Debug, Clone, Default, PartialEq, Eq, Builder)]
#[builder(setter(into))]
pub struct Socials {
    #[builder(default = "None")]
    facebook: Option<Handler>,
    #[builder(default = "None")]
    instagram: Option<Handler>,
    #[builder(default = "None")]
    linkedin: Option<Handler>,
    #[builder(default = "None")]
    twitter: Option<Handler>,
    #[builder(default = "None")]
    youtube: Option<Handler>,
}

impl Socials {
    /// Returns the facebook handler
    pub fn facebook(&self) -> Option<&Handler> {
        self.facebook.as_ref()
    }

    /// Returns the instagram handler
    pub fn instagram(&self) -> Option<&Handler> {
        self.instagram.as_ref()
    }

    /// Returns the linkedin handler
    pub fn linkedin(&self) -> Option<&Handler> {
        self.linkedin.as_ref()
    }

    /// Returns the twitter handler
    pub fn twitter(&self) -> Option<&Handler> {
        self.twitter.as_ref()
    }

    /// Returns the youtube handler
    pub fn youtube(&self) -> Option<&Handler> {
        self.youtube.as_ref()
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Handler(String);

impl TryFrom<&str> for Handler {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(Handler(String::from(value)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod socials {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_socials_value() {
            let social = SocialsBuilder::default()
                .facebook("facebook_user".try_into().ok())
                .instagram("instagram_user".try_into().ok())
                .linkedin("linkedin_user".try_into().ok())
                .twitter("twitter_user".try_into().ok())
                .youtube("youtube_user".try_into().ok())
                .build()
                .unwrap();

            assert_eq!(
                &Handler("facebook_user".to_string()),
                social.facebook().unwrap()
            );
            assert_eq!(
                &Handler("instagram_user".to_string()),
                social.instagram().unwrap()
            );
            assert_eq!(
                &Handler("linkedin_user".to_string()),
                social.linkedin().unwrap()
            );
            assert_eq!(
                &Handler("twitter_user".to_string()),
                social.twitter().unwrap()
            );
            assert_eq!(
                &Handler("youtube_user".to_string()),
                social.youtube().unwrap()
            );
        }
    }
}
