use crate::coordinate::WorldCoordinate;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.z)
    }
}
