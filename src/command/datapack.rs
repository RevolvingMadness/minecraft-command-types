use crate::command::{Command, enums::datapack_list_type::DatapackListType};
use crate::option_write_chain;
use crate::snbt::SNBT;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DatapackLoadPriority {
    First,
    Last,
    Before(String),
    After(String),
}

impl Display for DatapackLoadPriority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::First => f.write_str("first"),
            Self::Last => f.write_str("last"),
            Self::Before(existing) => write!(f, "before {}", existing),
            Self::After(existing) => write!(f, "after {}", existing),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DatapackCommand {
    Disable(String),
    Enable(String, Option<DatapackLoadPriority>),
    List(Option<DatapackListType>),
    Create(String, SNBT),
}

impl Display for DatapackCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disable(name) => {
                write!(f, "disable {}", name)
            }
            Self::Enable(name, load_priority) => {
                write!(f, "enable {}", name)?;

                option_write_chain!(f, load_priority);

                Ok(())
            }
            Self::List(list_type) => {
                f.write_str("list")?;

                option_write_chain!(f, list_type);

                Ok(())
            }
            Self::Create(id, description) => {
                write!(f, "create {} {}", id, description)
            }
        }
    }
}

impl From<DatapackCommand> for Command {
    fn from(value: DatapackCommand) -> Self {
        Self::Datapack(value)
    }
}
