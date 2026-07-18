use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameruleValue {
    Integer(i32),
    Boolean(bool),
}

impl Display for GameruleValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => value.fmt(f),
            Self::Boolean(value) => value.fmt(f),
        }
    }
}
