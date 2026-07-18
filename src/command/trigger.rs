use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TriggerAction {
    Add(i32),
    Set(i32),
}

impl Display for TriggerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(amount) => write!(f, "add {}", amount),
            Self::Set(amount) => write!(f, "set {}", amount),
        }
    }
}
