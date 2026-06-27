use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};
use std::ops::Range;

use crate::types::Float;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct IntegerRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

impl IntegerRange {
    #[must_use]
    pub fn new(min: Option<i32>, max: Option<i32>) -> Self {
        assert!(
            min.is_some() || max.is_some(),
            "min and/or max must be Some"
        );

        if let (Some(min), Some(max)) = (min, max)
            && min > max
        {
            panic!("min must be smaller or equal to max");
        }

        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn new_min(min: i32) -> Self {
        Self::new(Some(min), None)
    }

    #[inline]
    #[must_use]
    pub fn new_max(max: i32) -> Self {
        Self::new(None, Some(max))
    }

    #[inline]
    #[must_use]
    pub fn new_min_max(min: i32, max: i32) -> Self {
        Self::new(Some(min), Some(max))
    }

    #[inline]
    #[must_use]
    pub fn new_single(value: i32) -> Self {
        Self::new(Some(value), Some(value))
    }
}

impl Display for IntegerRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.min, self.max) {
            (Some(min), Some(max)) => {
                if min == max {
                    Display::fmt(&min, f)
                } else {
                    write!(f, "{}..{}", min, max)
                }
            }
            (Some(min), None) => {
                write!(f, "{}..", min)
            }
            (None, Some(max)) => {
                write!(f, "..{}", max)
            }
            (None, None) => {
                panic!("min and/or max must be Some")
            }
        }
    }
}

impl From<(i32, i32)> for IntegerRange {
    fn from(value: (i32, i32)) -> Self {
        Self::new_min_max(value.0, value.1)
    }
}

impl From<(Option<i32>, i32)> for IntegerRange {
    fn from(value: (Option<i32>, i32)) -> Self {
        Self::new(value.0, Some(value.1))
    }
}

impl From<(i32, Option<i32>)> for IntegerRange {
    fn from(value: (i32, Option<i32>)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl From<(Option<i32>, Option<i32>)> for IntegerRange {
    fn from(value: (Option<i32>, Option<i32>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Range<i32>> for IntegerRange {
    fn from(value: Range<i32>) -> Self {
        Self::new_min_max(value.start, value.end)
    }
}

impl From<Range<Option<i32>>> for IntegerRange {
    fn from(value: Range<Option<i32>>) -> Self {
        Self::new(value.start, value.end)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct FloatRange {
    pub min: Option<Float>,
    pub max: Option<Float>,
}

impl FloatRange {
    #[must_use]
    pub fn new(min: Option<Float>, max: Option<Float>) -> Self {
        assert!(
            min.is_some() || max.is_some(),
            "min and/or max must be Some"
        );

        if let (Some(min), Some(max)) = (min, max)
            && min > max
        {
            panic!("min must be smaller or equal to max");
        }

        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn new_min(min: Float) -> Self {
        Self::new(Some(min), None)
    }

    #[inline]
    #[must_use]
    pub fn new_max(max: Float) -> Self {
        Self::new(None, Some(max))
    }

    #[inline]
    #[must_use]
    pub fn new_min_max(min: Float, max: Float) -> Self {
        Self::new(Some(min), Some(max))
    }

    #[inline]
    #[must_use]
    pub fn new_single(value: Float) -> Self {
        Self::new(Some(value), Some(value))
    }
}

impl Display for FloatRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.min, self.max) {
            (Some(min), Some(max)) => {
                if min == max {
                    min.fmt(f)
                } else {
                    write!(f, "{}..{}", min, max)
                }
            }
            (Some(min), None) => {
                write!(f, "{}..", min)
            }
            (None, Some(max)) => {
                write!(f, "..{}", max)
            }
            (None, None) => {
                panic!("min and/or max must be Some")
            }
        }
    }
}

impl From<(Float, Float)> for FloatRange {
    fn from(value: (Float, Float)) -> Self {
        Self::new_min_max(value.0, value.1)
    }
}

impl From<(Option<Float>, Float)> for FloatRange {
    fn from(value: (Option<Float>, Float)) -> Self {
        Self::new(value.0, Some(value.1))
    }
}

impl From<(Float, Option<Float>)> for FloatRange {
    fn from(value: (Float, Option<Float>)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl From<(Option<Float>, Option<Float>)> for FloatRange {
    fn from(value: (Option<Float>, Option<Float>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Range<Float>> for FloatRange {
    fn from(value: Range<Float>) -> Self {
        Self::new_min_max(value.start, value.end)
    }
}

impl From<Range<Option<Float>>> for FloatRange {
    fn from(value: Range<Option<Float>>) -> Self {
        Self::new(value.start, value.end)
    }
}
