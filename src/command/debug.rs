use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DebugCommandType {
    Start,
    Stop,
    Function(ResourceLocation),
}

impl Display for DebugCommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugCommandType::Start => "start".fmt(f),
            DebugCommandType::Stop => "stop".fmt(f),
            DebugCommandType::Function(location) => write!(f, "function {}", location),
        }
    }
}
