use crate::command::enums::datapack_list_type::DatapackListType;
use crate::snbt::SNBT;
use minecraft_command_types_derive::HasMacro;
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
            DatapackLoadPriority::First => f.write_str("first"),
            DatapackLoadPriority::Last => f.write_str("last"),
            DatapackLoadPriority::Before(existing) => write!(f, "before {}", existing),
            DatapackLoadPriority::After(existing) => write!(f, "after {}", existing),
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
            DatapackCommand::Disable(name) => {
                write!(f, "disable {}", name)
            }
            DatapackCommand::Enable(name, load_priority) => {
                write!(f, "enable {}", name)?;

                if let Some(load_priority) = load_priority {
                    write!(f, " {}", load_priority)?;
                }

                Ok(())
            }
            DatapackCommand::List(list_type) => {
                f.write_str("list")?;

                if let Some(list_type) = list_type {
                    write!(f, " {}", list_type)?;
                }

                Ok(())
            }
            DatapackCommand::Create(id, description) => {
                write!(f, "create {} {}", id, description)
            }
        }
    }
}
