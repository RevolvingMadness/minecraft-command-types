use crate::has_macro::HasMacro;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum LocateType {
    Structure,
    Biome,
    POI,
}

impl Display for LocateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocateType::Structure => "structure".fmt(f),
            LocateType::Biome => "biome".fmt(f),
            LocateType::POI => "poi".fmt(f),
        }
    }
}
