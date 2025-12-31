use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct Rotation(pub NotNan<f32>, pub NotNan<f32>);

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
