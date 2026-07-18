use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LocateType {
    Structure,
    Biome,
    POI,
}

impl Display for LocateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Structure => f.write_str("structure"),
            Self::Biome => f.write_str("biome"),
            Self::POI => f.write_str("poi"),
        }
    }
}
