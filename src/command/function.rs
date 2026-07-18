use crate::{
    command::data::DataTarget,
    nbt_path::NbtPath,
    option_write_chain,
    snbt::{SNBTCompound, fmt_snbt_compound},
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionCommandArguments {
    Compound(SNBTCompound),
    DataTarget(DataTarget, Option<NbtPath>),
}

impl Display for FunctionCommandArguments {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
