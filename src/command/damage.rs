use crate::entity_selector::EntitySelector;
use crate::{coordinate::Coordinates, option_write_chain};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
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
