use crate::coordinate::WorldCoordinate;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct ColumnPosition {
    pub x: WorldCoordinate,
    pub z: WorldCoordinate,
}

impl ColumnPosition {
    #[inline]
    #[must_use]
    pub fn new(x: WorldCoordinate, z: WorldCoordinate) -> ColumnPosition {
        ColumnPosition { x, z }
    }
}

impl Display for ColumnPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.z)
    }
}
