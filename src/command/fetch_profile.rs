use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum FetchProfileCommand {
    Name(String),
    Id(String),
}

impl Display for FetchProfileCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchProfileCommand::Name(name) => write!(f, "name {}", name),
            FetchProfileCommand::Id(id) => write!(f, "id {}", id),
        }
    }
}
