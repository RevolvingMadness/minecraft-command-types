use crate::{command::Command, entity_selector::EntitySelector};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WhitelistCommand {
    Add(EntitySelector),
    List,
    Off,
    On,
    Reload,
    Remove(EntitySelector),
}

impl Display for WhitelistCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(selector) => write!(f, "add {}", selector),
            Self::List => f.write_str("list"),
            Self::Off => f.write_str("off"),
            Self::On => f.write_str("on"),
            Self::Reload => f.write_str("reload"),
            Self::Remove(selector) => write!(f, "remove {}", selector),
        }
    }
}

impl From<WhitelistCommand> for Command {
    fn from(value: WhitelistCommand) -> Self {
        Self::Whitelist(value)
    }
}

impl WhitelistCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::List => false,
            Self::Off => true,
            Self::On => true,
            Self::Reload => true,
            Self::Remove(..) => true,
        }
    }
}
