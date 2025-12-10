use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum StopwatchCommand {
    Create(ResourceLocation),
    Query(ResourceLocation, Option<NotNan<f32>>),
    Restart(ResourceLocation),
    Remove(ResourceLocation),
}

impl Display for StopwatchCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StopwatchCommand::Create(location) => {
                write!(f, "create {}", location)
            }
            StopwatchCommand::Query(location, scale) => {
                write!(f, "query {}", location)?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            StopwatchCommand::Restart(location) => {
                write!(f, "restart {}", location)
            }
            StopwatchCommand::Remove(location) => {
                write!(f, "remove {}", location)
            }
        }
    }
}
