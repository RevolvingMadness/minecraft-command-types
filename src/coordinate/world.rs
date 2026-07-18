use std::fmt::{self, Display, Formatter};

use crate::types::Double;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldCoordinate {
    Relative(Option<Double>),
    Absolute(Double),
}

impl WorldCoordinate {
    #[inline]
    #[must_use]
    pub const fn default_relative() -> Self {
        Self::Relative(None)
    }

    #[inline]
    #[must_use]
    pub fn default_absolute() -> Self {
        Self::Absolute(Double::default())
    }
}

impl Display for WorldCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relative(offset) => {
                if let Some(offset) = offset {
                    write!(f, "~{}", offset)
                } else {
                    write!(f, "~")
                }
            }
            Self::Absolute(value) => {
                write!(f, "{}", value)
            }
        }
    }
}
