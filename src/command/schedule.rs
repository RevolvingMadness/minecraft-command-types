use crate::command::enums::schedule_mode::ScheduleMode;
use crate::resource_location::ResourceLocation;
use crate::time::Time;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScheduleCommand {
    Function(ResourceLocation, Time, Option<ScheduleMode>),
    Clear(ResourceLocation),
}

impl Display for ScheduleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScheduleCommand::Function(location, time, mode) => {
                write!(f, "function {} {}", location, time)?;

                if let Some(mode) = mode {
                    write!(f, " {}", mode)?;
                }

                Ok(())
            }
            ScheduleCommand::Clear(location) => write!(f, "clear {}", location),
        }
    }
}
