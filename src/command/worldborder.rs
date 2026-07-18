use crate::{
    column_position::ColumnPosition,
    command::Command,
    option_write_chain,
    time::Time,
    types::{Double, Float},
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DamageWorldborderCommand {
    Amount(Float),
    Buffer(Float),
}

impl Display for DamageWorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Amount(amount) => write!(f, "amount {}", amount),
            Self::Buffer(buffer) => write!(f, "buffer {}", buffer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WarningWorldborderCommand {
    Distance(i32),
    Time(Time),
}

impl Display for WarningWorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Distance(distance) => write!(f, "distance {}", distance),
            Self::Time(time) => write!(f, "time {}", time),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorldborderCommand {
    Add(Double, Option<Time>),
    Center(ColumnPosition),
    Damage(DamageWorldborderCommand),
    Get,
    Set(Double, Option<Time>),
    Warning(WarningWorldborderCommand),
}

impl Display for WorldborderCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl WorldborderCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::Center(..) => true,
            Self::Damage(..) => true,
            Self::Get => false,
            Self::Set(..) => true,
            Self::Warning(..) => true,
        }
    }
}
