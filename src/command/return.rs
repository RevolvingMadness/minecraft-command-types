use crate::command::Command;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ReturnCommand {
    Value(i32),
    Fail,
    Run(Box<Command>),
}

impl Display for ReturnCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Fail => f.write_str("fail"),
            Self::Run(command) => write!(f, "run {}", command),
        }
    }
}

impl From<ReturnCommand> for Command {
    fn from(value: ReturnCommand) -> Self {
        Self::Return(value)
    }
}

impl ReturnCommand {
    pub const VALUE_0: Self = Self::Value(0);
    pub const VALUE_1: Self = Self::Value(1);
    pub const FAIL: Self = Self::Fail;
}
