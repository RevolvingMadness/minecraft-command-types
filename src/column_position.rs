use crate::coordinate::world::WorldCoordinate;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnPosition {
    pub x: WorldCoordinate,
    pub z: WorldCoordinate,
}

impl ColumnPosition {
    #[must_use]
    pub const fn new(x: WorldCoordinate, z: WorldCoordinate) -> Self {
        Self { x, z }
    }
}

impl Display for ColumnPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.z)
    }
}
