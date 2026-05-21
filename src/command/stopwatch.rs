use crate::resource_location::ResourceLocation;
use minecraft_command_types_procederal_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum StopwatchCommand {
    Create(ResourceLocation),
    Query(ResourceLocation, Option<NotNan<f32>>),
    Restart(ResourceLocation),
    Remove(ResourceLocation),
}

impl Display for StopwatchCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create(location) => {
                write!(f, "create {}", location)
            }
            Self::Query(location, scale) => {
                write!(f, "query {}", location)?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            Self::Restart(location) => {
                write!(f, "restart {}", location)
            }
            Self::Remove(location) => {
                write!(f, "remove {}", location)
            }
        }
    }
}
