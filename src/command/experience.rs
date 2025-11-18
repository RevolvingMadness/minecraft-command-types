use crate::entity_selector::EntitySelector;
use crate::enums::experience_type::ExperienceType;
use crate::has_macro::HasMacro;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ExperienceCommand {
    Add(EntitySelector, i32, ExperienceType),
    Set(EntitySelector, i32, ExperienceType),
    Query(EntitySelector, ExperienceType),
}

impl Display for ExperienceCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExperienceCommand::Add(selector, amount, experience_type) => {
                write!(f, "add {} {} {}", selector, amount, experience_type)
            }
            ExperienceCommand::Set(selector, amount, experience_type) => {
                write!(f, "set {} {} {}", selector, amount, experience_type)
            }
            ExperienceCommand::Query(selector, experience_type) => {
                write!(f, "query {} {}", selector, experience_type)
            }
        }
    }
}
