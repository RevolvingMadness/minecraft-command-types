use std::fmt::{self, Display, Formatter};

use crate::{
    command::enums::entity_anchor::EntityAnchor, coordinate::Coordinates,
    entity_selector::EntitySelector,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Facing {
    Position(Coordinates),
    Entity(EntitySelector, EntityAnchor),
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::Entity(selector, anchor) => write!(f, "entity {} {}", selector, anchor),
        }
    }
}
