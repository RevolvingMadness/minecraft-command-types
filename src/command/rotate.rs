use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::rotation::Rotation;
use crate::{command::enums::entity_anchor::EntityAnchor, option_write_chain};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum FacingRotateCommand {
    Coordinates(Coordinates),
    Entity(EntitySelector, Option<EntityAnchor>),
}

impl Display for FacingRotateCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinates(coordinates) => coordinates.fmt(f),
            Self::Entity(selector, anchor) => {
                write!(f, "entity {}", selector)?;

                option_write_chain!(f, anchor);

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum RotateCommand {
    Rotation(Rotation),
    Facing(FacingRotateCommand),
}

impl Display for RotateCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rotation(rotation) => rotation.fmt(f),
            Self::Facing(command) => {
                write!(f, "facing {}", command)
            }
        }
    }
}
