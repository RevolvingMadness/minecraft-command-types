use crate::entity_selector::EntitySelector;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum RideCommand {
    Mount(EntitySelector),
    Dismount,
}

impl Display for RideCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mount(selector) => write!(f, "mount {}", selector),
            Self::Dismount => f.write_str("dismount"),
        }
    }
}
