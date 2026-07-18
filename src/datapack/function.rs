use std::fmt::{self, Display, Formatter};

use crate::command::Command;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Function {
    pub commands: Vec<Command>,
}

impl Function {
    #[inline]
    pub fn add_command<C: Into<Command>>(&mut self, command: C) {
        let command = command.into();

        self.commands.push(command);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for command in &self.commands {
            writeln!(f, "{}", command)?;
        }

        Ok(())
    }
}
