use crate::command::enums::time_query_type::TimeQueryType;
use crate::command::{Command, enums::time_of_day::TimeOfDay};
use crate::time::Time;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TimeSetType {
    Time(Time),
    TimeOfDay(TimeOfDay),
}

impl Display for TimeSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Time(time) => time.fmt(f),
            Self::TimeOfDay(time_of_day) => time_of_day.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum TimeCommand {
    Add(Time),
    Query(TimeQueryType),
    Set(TimeSetType),
}

impl Display for TimeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(time) => write!(f, "add {}", time),
            Self::Query(query_type) => write!(f, "query {}", query_type),
            Self::Set(set_type) => write!(f, "set {}", set_type),
        }
    }
}

impl From<TimeCommand> for Command {
    fn from(value: TimeCommand) -> Self {
        Self::Time(value)
    }
}

impl TimeCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::Query(..) => false,
            Self::Set(..) => true,
        }
    }
}
