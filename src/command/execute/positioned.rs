use std::fmt::{self, Display, Formatter};

use crate::{
    command::enums::heightmap::Heightmap, coordinate::Coordinates, entity_selector::EntitySelector,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Positioned {
    Position(Coordinates),
    As(EntitySelector),
    Over(Heightmap),
}

impl Display for Positioned {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::As(selector) => write!(f, "as {}", selector),
            Self::Over(heightmap) => write!(f, "over {}", heightmap),
        }
    }
}
