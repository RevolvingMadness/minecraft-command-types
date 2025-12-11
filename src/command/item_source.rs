use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ItemSource {
    Block(Coordinates),
    Entity(EntitySelector),
}

impl Display for ItemSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemSource::Block(coords) => write!(f, "block {}", coords),
            ItemSource::Entity(selector) => write!(f, "entity {}", selector),
        }
    }
}
