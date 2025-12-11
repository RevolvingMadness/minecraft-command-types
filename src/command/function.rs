use crate::command::data::DataTarget;
use crate::nbt_path::{NbtPath, SNBTCompound};
use crate::snbt::fmt_snbt_compound;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum FunctionCommandArguments {
    Compound(SNBTCompound),
    DataTarget(DataTarget, Option<NbtPath>),
}

impl Display for FunctionCommandArguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionCommandArguments::Compound(compound) => fmt_snbt_compound(f, compound),
            FunctionCommandArguments::DataTarget(target, path) => {
                write!(f, "with {}", target)?;

                if let Some(path) = path {
                    write!(f, " {}", path)?;
                }

                Ok(())
            }
        }
    }
}
