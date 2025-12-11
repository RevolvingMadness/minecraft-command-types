use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum TriggerAction {
    Add(i32),
    Set(i32),
}

impl Display for TriggerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TriggerAction::Add(amount) => write!(f, "add {}", amount),
            TriggerAction::Set(amount) => write!(f, "set {}", amount),
        }
    }
}
