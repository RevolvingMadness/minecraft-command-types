use crate::resource_location::ResourceLocation;
use minecraft_command_types_procederal_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, HasMacro)]
pub enum CloneMaskMode {
    Replace,
    Masked,
    Filtered(ResourceLocation),
}

impl Display for CloneMaskMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Replace => write!(f, "replace"),
            Self::Masked => write!(f, "masked"),
            Self::Filtered(block) => write!(f, "filtered {}", block),
        }
    }
}
