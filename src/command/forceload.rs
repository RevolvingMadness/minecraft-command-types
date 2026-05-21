use crate::column_position::ColumnPosition;
use minecraft_command_types_procederal_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ForceloadRemoveType {
    ColumnPosition(ColumnPosition, Option<ColumnPosition>),
    All,
}

impl Display for ForceloadRemoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColumnPosition(from, to) => {
                from.fmt(f)?;

                if let Some(to) = to {
                    write!(f, " {}", to)?;
                }

                Ok(())
            }
            Self::All => f.write_str("all"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ForceloadCommand {
    Add(ColumnPosition, Option<ColumnPosition>),
    Remove(ForceloadRemoveType),
    Query(Option<ColumnPosition>),
}

impl Display for ForceloadCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(from, to) => {
                write!(f, "add {}", from)?;

                if let Some(to) = to {
                    write!(f, " {}", to)?;
                }

                Ok(())
            }
            Self::Remove(remove_type) => {
                write!(f, "remove {}", remove_type)
            }
            Self::Query(position) => {
                f.write_str("query")?;

                if let Some(position) = position {
                    write!(f, " {}", position)?;
                }

                Ok(())
            }
        }
    }
}
