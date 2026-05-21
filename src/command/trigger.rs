use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TriggerAction {
    Add(i32),
    Set(i32),
}

impl Display for TriggerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(amount) => write!(f, "add {}", amount),
            Self::Set(amount) => write!(f, "set {}", amount),
        }
    }
}
