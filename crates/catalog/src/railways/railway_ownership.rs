use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Ownership {
    Private,
    Public,
}

impl Display for Ownership {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod ownerships {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_display_ownerships() {
            assert_eq!("Private", Ownership::Private.to_string());
        }
    }
}
