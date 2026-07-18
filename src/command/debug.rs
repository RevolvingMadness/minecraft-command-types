use crate::{command::Command, resource_location::ResourceLocation};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum DebugCommand {
    Start,
    Stop,
    Function(ResourceLocation),
}

impl Display for DebugCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => f.write_str("start"),
            Self::Stop => f.write_str("stop"),
            Self::Function(location) => write!(f, "function {}", location),
        }
    }
}

impl From<DebugCommand> for Command {
    fn from(value: DebugCommand) -> Self {
        Self::Debug(value)
    }
}
