use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum CloneMaskMode {
    Replace,
    Masked,
    Filtered(ResourceLocation),
}

impl Display for CloneMaskMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CloneMaskMode::Replace => write!(f, "replace"),
            CloneMaskMode::Masked => write!(f, "masked"),
            CloneMaskMode::Filtered(block) => write!(f, "filtered {}", block),
        }
    }
}
