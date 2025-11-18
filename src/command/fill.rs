use crate::block::BlockPredicate;
use crate::enums::fill_mode::FillMode;
use crate::enums::fill_replace_mode::FillReplaceMode;
use crate::has_macro::HasMacro;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum FillCommand {
    Mode(FillMode),
    Replace(BlockPredicate, Option<FillReplaceMode>),
}

impl Display for FillCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FillCommand::Mode(mode) => mode.fmt(f),
            FillCommand::Replace(predicate, replace_mode) => {
                predicate.fmt(f)?;

                if let Some(replace_mode) = replace_mode {
                    write!(f, " {}", replace_mode)?;
                }

                Ok(())
            }
        }
    }
}
