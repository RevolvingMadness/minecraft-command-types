use crate::block::BlockState;
use crate::command::enums::fill_mode::FillMode;
use crate::command::enums::fill_replace_mode::FillReplaceMode;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum FillCommand {
    Mode(FillMode),
    Replace(BlockState, Option<FillReplaceMode>),
}

impl Display for FillCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mode(mode) => mode.fmt(f),
            Self::Replace(predicate, replace_mode) => {
                predicate.fmt(f)?;

                if let Some(replace_mode) = replace_mode {
                    write!(f, " {}", replace_mode)?;
                }

                Ok(())
            }
        }
    }
}
