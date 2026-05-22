use crate::command::{Command, enums::schedule_mode::ScheduleMode};
use crate::option_write_chain;
use crate::resource_location::ResourceLocation;
use crate::time::Time;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ScheduleCommand {
    Function(ResourceLocation, Time, Option<ScheduleMode>),
    Clear(ResourceLocation),
}

impl Display for ScheduleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(location, time, mode) => {
                write!(f, "function {} {}", location, time)?;

                option_write_chain!(f, mode);

                Ok(())
            }
            Self::Clear(location) => write!(f, "clear {}", location),
        }
    }
}

impl From<ScheduleCommand> for Command {
    fn from(value: ScheduleCommand) -> Self {
        Self::Schedule(value)
    }
}
