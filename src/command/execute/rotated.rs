use std::fmt::{self, Display, Formatter};

use crate::{entity_selector::EntitySelector, rotation::Rotation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rotated {
    Rotation(Rotation),
    As(EntitySelector),
}

impl Display for Rotated {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rotation(rotation) => rotation.fmt(f),
            Self::As(selector) => write!(f, "as {}", selector),
        }
    }
}
