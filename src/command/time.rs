use crate::command::enums::time_of_day::TimeOfDay;
use crate::command::enums::time_query_type::TimeQueryType;
use crate::time::Time;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TimeSetType {
    Time(Time),
    TimeOfDay(TimeOfDay),
}

impl Display for TimeSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeSetType::Time(time) => time.fmt(f),
            TimeSetType::TimeOfDay(time_of_day) => time_of_day.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TimeCommand {
    Add(Time),
    Query(TimeQueryType),
    Set(TimeSetType),
}

impl Display for TimeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeCommand::Add(time) => write!(f, "add {}", time),
            TimeCommand::Query(query_type) => write!(f, "query {}", query_type),
            TimeCommand::Set(set_type) => write!(f, "set {}", set_type),
        }
    }
}
