use crate::time::Time;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum AdvanceTimeTickCommand {
    Time(Option<Time>),
    Stop,
}

impl Display for AdvanceTimeTickCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Time(time) => {
                if let Some(time) = time {
                    write!(f, " {}", time)?;
                }

                Ok(())
            }
            Self::Stop => f.write_str(" stop"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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
            Self::Query => f.write_str("query"),
            Self::Rate(rate) => write!(f, "rate {}", rate),
            Self::Freeze => f.write_str("freeze"),
            Self::Unfreeze => f.write_str("unfreeze"),
            Self::Step(command) => write!(f, "step{}", command),
            Self::Sprint(command) => write!(f, "sprint{}", command),
        }
    }
}
