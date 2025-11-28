use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum TagCommand {
    Add(String),
    Remove(String),
    List,
}

impl Display for TagCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TagCommand::Add(tag) => write!(f, "add {}", tag),
            TagCommand::Remove(tag) => write!(f, "remove {}", tag),
            TagCommand::List => "list".fmt(f),
        }
    }
}
