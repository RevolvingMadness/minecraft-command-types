use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum LocateType {
    Structure,
    Biome,
    POI,
}

impl Display for LocateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Structure => f.write_str("structure"),
            Self::Biome => f.write_str("biome"),
            Self::POI => f.write_str("poi"),
        }
    }
}
