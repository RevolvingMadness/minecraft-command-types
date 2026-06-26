use crate::{column_position::ColumnPosition, command::Command, option_write_chain};
use minecraft_command_types_procedural_macros::HasMacro;
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

                option_write_chain!(f, to);

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

                option_write_chain!(f, to);

                Ok(())
            }
            Self::Remove(remove_type) => {
                write!(f, "remove {}", remove_type)
            }
            Self::Query(position) => {
                f.write_str("query")?;

                option_write_chain!(f, position);

                Ok(())
            }
        }
    }
}

impl From<ForceloadCommand> for Command {
    fn from(value: ForceloadCommand) -> Self {
        Self::Forceload(value)
    }
}

impl ForceloadCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::Remove(..) => true,
            Self::Query(..) => false,
        }
    }
}
