use crate::{command::Command, option_write_chain, time::Time, types::Float};
use minecraft_command_types_procedural_macros::HasMacro;
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
                option_write_chain!(f, time);

                Ok(())
            }
            Self::Stop => f.write_str(" stop"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TickCommand {
    Query,
    Rate(Float),
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

impl From<TickCommand> for Command {
    fn from(value: TickCommand) -> Self {
        Self::Tick(value)
    }
}

impl TickCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Query => false,
            Self::Rate(..) => true,
            Self::Freeze => true,
            Self::Unfreeze => true,
            Self::Step(..) => true,
            Self::Sprint(..) => true,
        }
    }
}
