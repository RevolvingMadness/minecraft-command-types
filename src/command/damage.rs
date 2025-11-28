use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DamageType {
    At(Coordinates),
    By(EntitySelector, Option<EntitySelector>),
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageType::At(coordinates) => write!(f, "at {}", coordinates),
            DamageType::By(by, from) => {
                write!(f, "by {}", by)?;

                if let Some(from) = from {
                    write!(f, " from {}", from)?;
                }

                Ok(())
            }
        }
    }
}
