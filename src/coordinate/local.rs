use std::fmt::{self, Display, Formatter};

use crate::types::Double;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct LocalCoordinate {
    pub offset: Double,
}

impl Display for LocalCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.offset == 0.0 {
            write!(f, "^")
        } else {
            write!(f, "^{}", self.offset)
        }
    }
}
