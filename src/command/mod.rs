mod advancement;
mod attribute;
mod permission_level;

use crate::command::advancement::AdvancementCommand;
use crate::command::attribute::AttributeCommand;
use crate::command::permission_level::PermissionLevel;
use crate::entity_selector::EntitySelector;
use crate::enums::advancement_type::AdvancementType;
use crate::resource_location::ResourceLocation;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    Advancement(AdvancementType, EntitySelector, AdvancementCommand),
    Attribute(EntitySelector, ResourceLocation, AttributeCommand),
}

impl Command {
    pub fn get_permission_level(&self) -> PermissionLevel {
        match self {
            Command::Advancement(..) | Command::Attribute(..) => {
                PermissionLevel::try_from(2).unwrap()
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Advancement(type_, selector, command) => {
                write!(f, "{} {} {}", type_, selector, command)
            }
            Command::Attribute(selector, attribute, command) => {
                write!(f, "{} {} {}", selector, attribute, command)
            }
        }
    }
}
