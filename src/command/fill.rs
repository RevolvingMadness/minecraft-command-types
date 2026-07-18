use crate::command::enums::fill_mode::FillMode;
use crate::command::enums::fill_replace_mode::FillReplaceMode;
use crate::{block::BlockState, option_write_chain};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum FillCommand {
    Mode(FillMode),
    Replace(BlockState, Option<FillReplaceMode>),
}

impl Display for FillCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mode(mode) => mode.fmt(f),
            Self::Replace(predicate, replace_mode) => {
                predicate.fmt(f)?;

                option_write_chain!(f, replace_mode);

                Ok(())
            }
        }
    }
}
