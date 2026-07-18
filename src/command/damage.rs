use crate::{coordinate::Coordinates, entity_selector::EntitySelector, option_write_chain};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DamageType {
    At(Coordinates),
    By(EntitySelector, Option<EntitySelector>),
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::At(coordinates) => write!(f, "at {}", coordinates),
            Self::By(by, from) => {
                write!(f, "by {}", by)?;

                option_write_chain!(f, from);

                Ok(())
            }
        }
    }
}
