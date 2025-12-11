use crate::column_position::ColumnPosition;
use crate::time::Time;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DamageWorldborderCommand {
    Amount(NotNan<f32>),
    Buffer(NotNan<f32>),
}

impl Display for DamageWorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageWorldborderCommand::Amount(amount) => write!(f, "amount {}", amount),
            DamageWorldborderCommand::Buffer(buffer) => write!(f, "buffer {}", buffer),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum WarningWorldborderCommand {
    Distance(i32),
    Time(Time),
}

impl Display for WarningWorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningWorldborderCommand::Distance(distance) => write!(f, "distance {}", distance),
            WarningWorldborderCommand::Time(time) => write!(f, "time {}", time),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
            WorldborderCommand::Add(distance, time) => {
                write!(f, "add {}", distance)?;

                if let Some(time) = time {
                    write!(f, " {}", time)?;
                }

                Ok(())
            }
            WorldborderCommand::Center(position) => write!(f, "center {}", position),
            WorldborderCommand::Damage(damage_command) => write!(f, "damage {}", damage_command),
            WorldborderCommand::Get => f.write_str("get"),
            WorldborderCommand::Set(distance, time) => {
                write!(f, "set {}", distance)?;

                if let Some(time) = time {
                    write!(f, " {}", time)?;
                }

                Ok(())
            }
            WorldborderCommand::Warning(warning_command) => {
                write!(f, "warning {}", warning_command)
            }
        }
    }
}
