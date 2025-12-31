use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TimeSuffix {
    Days,
    Seconds,
    Ticks,
}

impl Display for TimeSuffix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeSuffix::Days => f.write_str("d"),
            TimeSuffix::Seconds => f.write_str("s"),
            TimeSuffix::Ticks => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct Time(NotNan<f32>, Option<TimeSuffix>);

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)?;

        if let Some(suffix) = &self.1 {
            suffix.fmt(f)?;
        }

        Ok(())
    }
}
