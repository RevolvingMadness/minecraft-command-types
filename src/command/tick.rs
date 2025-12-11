use crate::time::Time;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum AdvanceTimeTickCommand {
    Time(Option<Time>),
    Stop,
}

impl Display for AdvanceTimeTickCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvanceTimeTickCommand::Time(time) => {
                if let Some(time) = time {
                    write!(f, " {}", time)?;
                }

                Ok(())
            }
            AdvanceTimeTickCommand::Stop => f.write_str(" stop"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum TickCommand {
    Query,
    Rate(NotNan<f32>),
    Freeze,
    Unfreeze,
    Step(AdvanceTimeTickCommand),
    Sprint(AdvanceTimeTickCommand),
}

impl Display for TickCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TickCommand::Query => f.write_str("query"),
            TickCommand::Rate(rate) => write!(f, "rate {}", rate),
            TickCommand::Freeze => f.write_str("freeze"),
            TickCommand::Unfreeze => f.write_str("unfreeze"),
            TickCommand::Step(command) => write!(f, "step{}", command),
            TickCommand::Sprint(command) => write!(f, "sprint{}", command),
        }
    }
}
