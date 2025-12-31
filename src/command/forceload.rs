use crate::column_position::ColumnPosition;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ForceloadRemoveType {
    ColumnPosition(ColumnPosition, Option<ColumnPosition>),
    All,
}

impl Display for ForceloadRemoveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ForceloadRemoveType::ColumnPosition(from, to) => {
                from.fmt(f)?;

                if let Some(to) = to {
                    write!(f, " {}", to)?;
                }

                Ok(())
            }
            ForceloadRemoveType::All => f.write_str("all"),
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
            ForceloadCommand::Add(from, to) => {
                write!(f, "add {}", from)?;

                if let Some(to) = to {
                    write!(f, " {}", to)?;
                }

                Ok(())
            }
            ForceloadCommand::Remove(remove_type) => {
                write!(f, "remove {}", remove_type)
            }
            ForceloadCommand::Query(position) => {
                f.write_str("query")?;

                if let Some(position) = position {
                    write!(f, " {}", position)?;
                }

                Ok(())
            }
        }
    }
}
