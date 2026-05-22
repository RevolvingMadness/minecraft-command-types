use crate::command::{Command, enums::random_type::RandomType};
use crate::option_write_chain;
use crate::range::IntegerRange;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum RandomResetType {
    All,
    Sequence(ResourceLocation),
}

impl Display for RandomResetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => f.write_str("*"),
            Self::Sequence(sequence) => sequence.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum RandomCommand {
    ValueRoll(RandomType, IntegerRange, Option<ResourceLocation>),
    Reset(RandomResetType, Option<i32>, Option<bool>, Option<bool>),
}

impl Display for RandomCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueRoll(random_type, range, sequence) => {
                write!(f, "{} {}", random_type, range)?;

                option_write_chain!(f, sequence);

                Ok(())
            }
            Self::Reset(reset_type, seed, include_world_seed, include_sequence_id) => {
                write!(f, "reset {}", reset_type)?;

                option_write_chain!(f, seed, include_world_seed, include_sequence_id);

                Ok(())
            }
        }
    }
}

impl From<RandomCommand> for Command {
    fn from(value: RandomCommand) -> Self {
        Self::Random(value)
    }
}
