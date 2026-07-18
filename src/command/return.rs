use crate::command::Command;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReturnCommand {
    Value(i32),
    Fail,
    Run(Box<Command>),
}

impl Display for ReturnCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
