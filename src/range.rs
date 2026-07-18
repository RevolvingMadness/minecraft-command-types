use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};
use std::ops::Range as OpsRange;

use crate::types::Float;

pub type IntegerRange = Range<i32>;
pub type FloatRange = Range<Float>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, HasMacro)]
pub enum Range<T> {
    LowerBound(T),
    UpperBound(T),
    Bounds {
        lower: T,
        upper: T,
    },
    Single(T),
}

impl<T> Range<T> {
    #[inline]
    #[must_use]
    pub const fn new_lower(lower: T) -> Self {
        Self::LowerBound(lower)
    }

    #[inline]
    #[must_use]
    pub const fn new_upper(upper: T) -> Self {
        Self::UpperBound(upper)
    }

    #[inline]
    #[must_use]
    pub const fn new_single(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T: PartialEq + PartialOrd> Range<T> {
    #[must_use]
    pub fn new(lower: Option<T>, upper: Option<T>) -> Option<Self> {
        Some(match (lower, upper) {
            (None, None) => return None,
            (None, Some(upper)) => Self::UpperBound(upper),
            (Some(lower), None) => Self::LowerBound(lower),
            (Some(lower), Some(upper)) => {
                if lower == upper {
                    Self::Single(lower)
                } else {
                    if lower > upper {
                        return None;
                    }

                    Self::Bounds {
                        lower,
                        upper,
                    }
                }
            }
        })
    }

    #[inline]
    #[must_use]
    pub fn new_bounds(lower: T, upper: T) -> Option<Self> {
        Self::new(Some(lower), Some(upper))
    }
}

impl<T: Clone> Range<T> {
    #[must_use]
    pub fn lower(&self) -> Option<T> {
        Some(match self {
            Self::LowerBound(lower) => lower.clone(),
            Self::UpperBound(..) => return None,
            Self::Bounds {
                lower,
                ..
            } => lower.clone(),
            Self::Single(value) => value.clone(),
        })
    }

    #[must_use]
    pub fn upper(&self) -> Option<T> {
        Some(match self {
            Self::LowerBound(..) => return None,
            Self::UpperBound(upper) => upper.clone(),
            Self::Bounds {
                upper,
                ..
            } => upper.clone(),
            Self::Single(value) => value.clone(),
        })
    }
}

impl<T: Display> Display for Range<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::LowerBound(lower) => write!(f, "{}..", lower),
            Self::UpperBound(upper) => write!(f, "..{}", upper),
            Self::Bounds {
                lower,
                upper,
            } => write!(f, "{}..{}", lower, upper),
            Self::Single(value) => value.fmt(f),
        }
    }
}

impl<T> From<(T, T)> for Range<T> {
    fn from((lower, upper): (T, T)) -> Self {
        Self::Bounds {
            lower,
            upper,
        }
    }
}

impl<T: PartialEq + PartialOrd> TryFrom<(Option<T>, T)> for Range<T> {
    type Error = ();

    fn try_from((lower, upper): (Option<T>, T)) -> Result<Self, Self::Error> {
        Self::new(lower, Some(upper)).ok_or(())
    }
}

impl<T: PartialEq + PartialOrd> TryFrom<(T, Option<T>)> for Range<T> {
    type Error = ();

    fn try_from((lower, upper): (T, Option<T>)) -> Result<Self, Self::Error> {
        Self::new(Some(lower), upper).ok_or(())
    }
}

impl<T: PartialEq + PartialOrd> TryFrom<(Option<T>, Option<T>)> for Range<T> {
    type Error = ();

    fn try_from(value: (Option<T>, Option<T>)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1).ok_or(())
    }
}

impl<T: PartialEq + PartialOrd> TryFrom<OpsRange<T>> for Range<T> {
    type Error = ();

    fn try_from(
        OpsRange {
            start: lower,
            end: upper,
        }: OpsRange<T>,
    ) -> Result<Self, Self::Error> {
        Self::new_bounds(lower, upper).ok_or(())
    }
}

impl<T: PartialEq + PartialOrd> TryFrom<OpsRange<Option<T>>> for Range<T> {
    type Error = ();

    fn try_from(
        OpsRange {
            start: lower,
            end: upper,
        }: OpsRange<Option<T>>,
    ) -> Result<Self, Self::Error> {
        Self::new(lower, upper).ok_or(())
    }
}

impl<T> From<T> for Range<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}
