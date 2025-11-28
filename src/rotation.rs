use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, HasMacro)]
pub struct Rotation(NotNan<f32>, NotNan<f32>);

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}
