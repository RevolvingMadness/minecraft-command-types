use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TriggerAction {
    Add(i32),
    Set(i32),
}

impl Display for TriggerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(amount) => write!(f, "add {}", amount),
            Self::Set(amount) => write!(f, "set {}", amount),
        }
    }
}
