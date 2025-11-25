use crate::has_macro::HasMacro;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum GameruleValue {
    Integer(i32),
    Boolean(bool),
}

impl Display for GameruleValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameruleValue::Integer(value) => value.fmt(f),
            GameruleValue::Boolean(value) => value.fmt(f),
        }
    }
}
