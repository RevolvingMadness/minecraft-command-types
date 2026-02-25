use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TagCommand {
    Add(String),
    Remove(String),
    List,
}

impl Display for TagCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(tag) => write!(f, "add {}", tag),
            Self::Remove(tag) => write!(f, "remove {}", tag),
            Self::List => f.write_str("list"),
        }
    }
}
