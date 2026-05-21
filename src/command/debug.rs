use crate::resource_location::ResourceLocation;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DebugCommandType {
    Start,
    Stop,
    Function(ResourceLocation),
}

impl Display for DebugCommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => f.write_str("start"),
            Self::Stop => f.write_str("stop"),
            Self::Function(location) => write!(f, "function {}", location),
        }
    }
}
