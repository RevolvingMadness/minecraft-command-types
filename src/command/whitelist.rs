use crate::entity_selector::EntitySelector;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
            WhitelistCommand::Add(selector) => write!(f, "add {}", selector),
            WhitelistCommand::List => f.write_str("list"),
            WhitelistCommand::Off => f.write_str("off"),
            WhitelistCommand::On => f.write_str("on"),
            WhitelistCommand::Reload => f.write_str("reload"),
            WhitelistCommand::Remove(selector) => write!(f, "remove {}", selector),
        }
    }
}
