use crate::command::enums::entity_anchor::EntityAnchor;
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::rotation::Rotation;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TeleportFacing {
    Position(Coordinates),
    Entity(EntitySelector, Option<EntityAnchor>),
}

impl Display for TeleportFacing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::Entity(selector, anchor) => {
                write!(f, "entity {}", selector)?;

                if let Some(anchor) = anchor {
                    write!(f, " {}", anchor)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TeleportCoordinatesType {
    Rotation(Rotation),
    Facing(TeleportFacing),
}

impl Display for TeleportCoordinatesType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rotation(rotation) => rotation.fmt(f),
            Self::Facing(facing) => {
                write!(f, "facing {}", facing)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TargetTeleportCommand {
    Coordinates(Coordinates, Option<TeleportCoordinatesType>),
    Entity(EntitySelector),
}

impl Display for TargetTeleportCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coordinates(coordinates, additional) => {
                coordinates.fmt(f)?;

                if let Some(additional) = additional {
                    write!(f, " {}", additional)?;
                }

                Ok(())
            }
            Self::Entity(selector) => selector.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TeleportCommand {
    Coordinates(Coordinates),
    Entity(EntitySelector, Option<TargetTeleportCommand>),
}

impl Display for TeleportCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coordinates(coordinates) => coordinates.fmt(f),
            Self::Entity(selector, additional) => {
                selector.fmt(f)?;

                if let Some(additional) = additional {
                    write!(f, " {}", additional)?;
                }

                Ok(())
            }
        }
    }
}
