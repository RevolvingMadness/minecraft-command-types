use crate::command::data::DataTarget;
use crate::nbt_path::{NbtPath, SNBTCompound};
use crate::option_write_chain;
use crate::snbt::fmt_snbt_compound;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum FunctionCommandArguments {
    Compound(SNBTCompound),
    DataTarget(DataTarget, Option<NbtPath>),
}

impl Display for FunctionCommandArguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compound(compound) => fmt_snbt_compound(f, compound),
            Self::DataTarget(target, path) => {
                write!(f, "with {}", target)?;

                option_write_chain!(f, path);

                Ok(())
            }
        }
    }
}
