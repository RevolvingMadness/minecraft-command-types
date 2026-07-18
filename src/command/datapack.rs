use crate::{
    command::{Command, enums::datapack_list_type::DatapackListType},
    option_write_chain,
    snbt::Snbt,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatapackLoadPriority {
    First,
    Last,
    Before(String),
    After(String),
}

impl Display for DatapackLoadPriority {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::First => f.write_str("first"),
            Self::Last => f.write_str("last"),
            Self::Before(existing) => write!(f, "before {}", existing),
            Self::After(existing) => write!(f, "after {}", existing),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatapackCommand {
    Disable(String),
    Enable(String, Option<DatapackLoadPriority>),
    List(Option<DatapackListType>),
    Create(String, Snbt),
}

impl Display for DatapackCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl DatapackCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Disable(..) => true,
            Self::Enable(..) => true,
            Self::List(..) => false,
            Self::Create(..) => true,
        }
    }
}
