use minecraft_command_types_derive::HasMacro;
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
            LocateType::Structure => f.write_str("structure"),
            LocateType::Biome => f.write_str("biome"),
            LocateType::POI => f.write_str("poi"),
        }
    }
}
