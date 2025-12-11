use crate::command::Command;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ReturnCommand {
    Value(i32),
    Fail,
    Run(Box<Command>),
}

impl Display for ReturnCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnCommand::Value(v) => write!(f, "{}", v),
            ReturnCommand::Fail => f.write_str("fail"),
            ReturnCommand::Run(command) => write!(f, "run {}", command),
        }
    }
}
