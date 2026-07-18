use crate::command::{Command, enums::experience_type::ExperienceType};
use crate::entity_selector::EntitySelector;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum ExperienceCommand {
    Add(EntitySelector, i32, ExperienceType),
    Set(EntitySelector, i32, ExperienceType),
    Query(EntitySelector, ExperienceType),
}

impl Display for ExperienceCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(selector, amount, experience_type) => {
                write!(f, "add {} {} {}", selector, amount, experience_type)
            }
            Self::Set(selector, amount, experience_type) => {
                write!(f, "set {} {} {}", selector, amount, experience_type)
            }
            Self::Query(selector, experience_type) => {
                write!(f, "query {} {}", selector, experience_type)
            }
        }
    }
}

impl From<ExperienceCommand> for Command {
    fn from(value: ExperienceCommand) -> Self {
        Self::Experience(value)
    }
}

impl ExperienceCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::Set(..) => true,
            Self::Query(..) => false,
        }
    }
}
