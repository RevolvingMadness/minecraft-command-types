use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LocateType {
    Structure,
    Biome,
    PointOfInterest,
}

impl Display for LocateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Structure => f.write_str("structure"),
            Self::Biome => f.write_str("biome"),
            Self::PointOfInterest => f.write_str("poi"),
        }
    }
}
