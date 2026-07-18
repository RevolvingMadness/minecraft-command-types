use crate::{
    command::{Command, enums::schedule_mode::ScheduleMode},
    option_write_chain,
    resource_location::ResourceLocation,
    time::Time,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleCommand {
    Function(ResourceLocation, Time, Option<ScheduleMode>),
    Clear(ResourceLocation),
}

impl Display for ScheduleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
