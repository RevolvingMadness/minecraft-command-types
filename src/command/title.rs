use crate::command::enums::title_type::TitleType;
use crate::snbt::SNBT;
use crate::time::Time;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum TitleCommand {
    Clear,
    Reset,
    Title(TitleType, SNBT),
    Times(Time, Time, Time),
}

impl Display for TitleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear => f.write_str("clear"),
            Self::Reset => f.write_str("reset"),
            Self::Title(type_, title) => write!(f, "{} {}", type_, title),
            Self::Times(fade_in, stay, fade_out) => {
                write!(f, "times {} {} {}", fade_in, stay, fade_out)
            }
        }
    }
}
