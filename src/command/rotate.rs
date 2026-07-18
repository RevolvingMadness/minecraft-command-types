use crate::{
    command::enums::entity_anchor::EntityAnchor, coordinate::Coordinates,
    entity_selector::EntitySelector, option_write_chain, rotation::Rotation,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
