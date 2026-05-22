use crate::option_write_chain;
use crate::time::Time;
use crate::{column_position::ColumnPosition, command::Command};
use minecraft_command_types_procedural_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DamageWorldborderCommand {
    Amount(NotNan<f32>),
    Buffer(NotNan<f32>),
}

impl Display for DamageWorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amount(amount) => write!(f, "amount {}", amount),
            Self::Buffer(buffer) => write!(f, "buffer {}", buffer),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WarningWorldborderCommand {
    Distance(i32),
    Time(Time),
}

impl Display for WarningWorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Distance(distance) => write!(f, "distance {}", distance),
            Self::Time(time) => write!(f, "time {}", time),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WorldborderCommand {
    Add(NotNan<f64>, Option<Time>),
    Center(ColumnPosition),
    Damage(DamageWorldborderCommand),
    Get,
    Set(NotNan<f64>, Option<Time>),
    Warning(WarningWorldborderCommand),
}

impl Display for WorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(distance, time) => {
                write!(f, "add {}", distance)?;

                option_write_chain!(f, time);

                Ok(())
            }
            Self::Center(position) => write!(f, "center {}", position),
            Self::Damage(damage_command) => write!(f, "damage {}", damage_command),
            Self::Get => f.write_str("get"),
            Self::Set(distance, time) => {
                write!(f, "set {}", distance)?;

                option_write_chain!(f, time);

                Ok(())
            }
            Self::Warning(warning_command) => {
                write!(f, "warning {}", warning_command)
            }
        }
    }
}

impl From<WorldborderCommand> for Command {
    fn from(value: WorldborderCommand) -> Self {
        Self::Worldborder(value)
    }
}
