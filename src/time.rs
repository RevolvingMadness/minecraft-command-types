use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

use crate::types::Float;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TimeSuffix {
    Days,
    Seconds,
    Ticks,
}

impl Display for TimeSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Days => f.write_str("d"),
            Self::Seconds => f.write_str("s"),
            Self::Ticks => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
pub struct Time(Float, Option<TimeSuffix>);

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;

        if let Some(suffix) = &self.1 {
            suffix.fmt(f)?;
        }

        Ok(())
    }
}
