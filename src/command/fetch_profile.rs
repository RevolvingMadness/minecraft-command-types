use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

use crate::command::Command;

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum FetchProfileCommand {
    Name(String),
    Id(String),
}

impl Display for FetchProfileCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "name {}", name),
            Self::Id(id) => write!(f, "id {}", id),
        }
    }
}

impl From<FetchProfileCommand> for Command {
    fn from(value: FetchProfileCommand) -> Self {
        Self::FetchProfile(value)
    }
}
