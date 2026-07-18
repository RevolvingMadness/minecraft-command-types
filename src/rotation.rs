use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

use crate::types::Float;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
pub struct Rotation(pub Float, pub Float);

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
