use crate::entity_selector::EntitySelector;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RideCommand {
    Mount(EntitySelector),
    Dismount,
}

impl Display for RideCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mount(selector) => write!(f, "mount {}", selector),
            Self::Dismount => f.write_str("dismount"),
        }
    }
}
