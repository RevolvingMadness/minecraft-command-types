use crate::{coordinate::Coordinates, entity_selector::EntitySelector};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemSource {
    Block(Coordinates),
    Entity(EntitySelector),
}

impl Display for ItemSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(coords) => write!(f, "block {}", coords),
            Self::Entity(selector) => write!(f, "entity {}", selector),
        }
    }
}
