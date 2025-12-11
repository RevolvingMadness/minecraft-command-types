use crate::entity_selector::EntitySelector;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum RideCommand {
    Mount(EntitySelector),
    Dismount,
}

impl Display for RideCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RideCommand::Mount(selector) => write!(f, "mount {}", selector),
            RideCommand::Dismount => f.write_str("dismount"),
        }
    }
}
