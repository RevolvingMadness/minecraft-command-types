use std::fmt::{self, Display, Formatter};

use crate::{
    command::{
        data::DataTarget,
        enums::{bossbar_store_type::BossbarStoreType, numeric_snbt_type::NumericSnbtType},
        execute::ExecuteSubcommand,
    },
    nbt_path::NbtPath,
    player_score::PlayerScore,
    resource_location::ResourceLocation,
    types::Float,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecuteStoreSubcommand {
    Data(
        DataTarget,
        NbtPath,
        NumericSnbtType,
        Float,
        Box<ExecuteSubcommand>,
    ),
    Bossbar(ResourceLocation, BossbarStoreType, Box<ExecuteSubcommand>),
    Score(PlayerScore, Box<ExecuteSubcommand>),
}

impl Display for ExecuteStoreSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Data(target, path, num_type, scale, next) => {
                write!(f, "{} {} {} {} {}", target, path, num_type, scale, next)?;

                Ok(())
            }
            Self::Bossbar(id, store_type, next) => {
                write!(f, "bossbar {} {} {}", id, store_type, next)?;

                Ok(())
            }
            Self::Score(score, next) => {
                write!(f, "score {} {}", score, next)?;

                Ok(())
            }
        }
    }
}

impl ExecuteStoreSubcommand {
    #[must_use]
    pub fn then(self, next: ExecuteSubcommand) -> Option<Self> {
        Some(match self {
            Self::Data(target, path, num_type, scale, inner_next) => Self::Data(
                target,
                path,
                num_type,
                scale,
                Box::new(inner_next.then(next)?),
            ),
            Self::Bossbar(id, store_type, inner_next) => {
                Self::Bossbar(id, store_type, Box::new(inner_next.then(next)?))
            }
            Self::Score(score, inner_next) => Self::Score(score, Box::new(inner_next.then(next)?)),
        })
    }
}
