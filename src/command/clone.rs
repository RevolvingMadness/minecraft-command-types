use crate::resource_location::ResourceLocation;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CloneMaskMode {
    Replace,
    Masked,
    Filtered(ResourceLocation),
}

impl Display for CloneMaskMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Replace => write!(f, "replace"),
            Self::Masked => write!(f, "masked"),
            Self::Filtered(block) => write!(f, "filtered {}", block),
        }
    }
}
