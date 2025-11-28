use crate::command::enums::random_type::RandomType;
use crate::has_macro::HasMacro;
use crate::range::IntegerRange;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum RandomResetType {
    All,
    Sequence(ResourceLocation),
}

impl Display for RandomResetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RandomResetType::All => "*".fmt(f),
            RandomResetType::Sequence(sequence) => sequence.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum RandomCommand {
    ValueRoll(RandomType, IntegerRange, Option<ResourceLocation>),
    Reset(RandomResetType, Option<i32>, Option<bool>, Option<bool>),
}

impl Display for RandomCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RandomCommand::ValueRoll(random_type, range, sequence) => {
                write!(f, "{} {}", random_type, range)?;

                if let Some(sequence) = sequence {
                    write!(f, " {}", sequence)?;
                }

                Ok(())
            }
            RandomCommand::Reset(reset_type, seed, include_world_seed, include_sequence_id) => {
                write!(f, "reset {}", reset_type)?;

                if let Some(seed) = seed {
                    write!(f, " {}", seed)?;

                    if let Some(include_world_seed) = include_world_seed {
                        write!(f, " {}", include_world_seed)?;

                        if let Some(include_sequence_id) = include_sequence_id {
                            write!(f, " {}", include_sequence_id)?;
                        }
                    }
                }

                Ok(())
            }
        }
    }
}
