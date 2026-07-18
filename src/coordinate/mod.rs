use std::fmt::{self, Display, Formatter};

use crate::coordinate::{local::LocalCoordinate, world::WorldCoordinate};

pub mod local;
pub mod world;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coordinates {
    World(WorldCoordinate, WorldCoordinate, WorldCoordinate),
    Local(LocalCoordinate, LocalCoordinate, LocalCoordinate),
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::World(x, y, z) => {
                write!(f, "{} {} {}", x, y, z)
            }
            Self::Local(x, y, z) => {
                write!(f, "{} {} {}", x, y, z)
            }
        }
    }
}

impl Coordinates {
    #[inline]
    #[must_use]
    pub const fn splat_world(coordinate: WorldCoordinate) -> Self {
        Self::World(coordinate, coordinate, coordinate)
    }

    #[inline]
    #[must_use]
    pub const fn splat_local(coordinate: LocalCoordinate) -> Self {
        Self::Local(coordinate, coordinate, coordinate)
    }

    #[inline]
    #[must_use]
    pub const fn world_default_relative() -> Self {
        Self::splat_world(WorldCoordinate::default_relative())
    }

    #[inline]
    #[must_use]
    pub fn world_default_absolute() -> Self {
        Self::splat_world(WorldCoordinate::default_absolute())
    }

    #[inline]
    #[must_use]
    pub fn local_default() -> Self {
        Self::splat_local(LocalCoordinate::default())
    }
}
