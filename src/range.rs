use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct IntegerRange {
    pub min: Option<i32>,
    pub max: Option<i32>,
}

impl IntegerRange {
    pub fn new(min: Option<i32>, max: Option<i32>) -> IntegerRange {
        if min.is_none() && max.is_none() {
            panic!("min and/or max must be Some")
        }

        if let (Some(min), Some(max)) = (min, max)
            && min > max
        {
            panic!("min must be smaller or equal to max");
        }

        IntegerRange { min, max }
    }

    pub fn new_min(min: i32) -> IntegerRange {
        Self::new(Some(min), None)
    }

    pub fn new_max(max: i32) -> IntegerRange {
        Self::new(None, Some(max))
    }

    pub fn new_min_max(min: i32, max: i32) -> IntegerRange {
        Self::new(Some(min), Some(max))
    }

    pub fn new_single(value: i32) -> IntegerRange {
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
        IntegerRange::new_min_max(value.0, value.1)
    }
}

impl From<(Option<i32>, i32)> for IntegerRange {
    fn from(value: (Option<i32>, i32)) -> Self {
        IntegerRange::new(value.0, Some(value.1))
    }
}

impl From<(i32, Option<i32>)> for IntegerRange {
    fn from(value: (i32, Option<i32>)) -> Self {
        IntegerRange::new(Some(value.0), value.1)
    }
}

impl From<(Option<i32>, Option<i32>)> for IntegerRange {
    fn from(value: (Option<i32>, Option<i32>)) -> Self {
        IntegerRange::new(value.0, value.1)
    }
}

impl From<Range<i32>> for IntegerRange {
    fn from(value: Range<i32>) -> Self {
        IntegerRange::new_min_max(value.start, value.end)
    }
}

impl From<Range<Option<i32>>> for IntegerRange {
    fn from(value: Range<Option<i32>>) -> Self {
        IntegerRange::new(value.start, value.end)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct FloatRange {
    pub min: Option<NotNan<f32>>,
    pub max: Option<NotNan<f32>>,
}

impl FloatRange {
    #[must_use]
    pub fn new(min: Option<NotNan<f32>>, max: Option<NotNan<f32>>) -> FloatRange {
        if min.is_none() && max.is_none() {
            panic!("min and/or max must be Some")
        }

        if let (Some(min), Some(max)) = (min, max)
            && min > max
        {
            panic!("min must be smaller or equal to max");
        }

        FloatRange { min, max }
    }

    #[inline]
    #[must_use]
    pub fn new_min(min: NotNan<f32>) -> FloatRange {
        Self::new(Some(min), None)
    }

    #[inline]
    #[must_use]
    pub fn new_max(max: NotNan<f32>) -> FloatRange {
        Self::new(None, Some(max))
    }

    #[inline]
    #[must_use]
    pub fn new_min_max(min: NotNan<f32>, max: NotNan<f32>) -> FloatRange {
        Self::new(Some(min), Some(max))
    }

    #[inline]
    #[must_use]
    pub fn new_single(value: NotNan<f32>) -> FloatRange {
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

impl From<(NotNan<f32>, NotNan<f32>)> for FloatRange {
    fn from(value: (NotNan<f32>, NotNan<f32>)) -> Self {
        FloatRange::new_min_max(value.0, value.1)
    }
}

impl From<(Option<NotNan<f32>>, NotNan<f32>)> for FloatRange {
    fn from(value: (Option<NotNan<f32>>, NotNan<f32>)) -> Self {
        FloatRange::new(value.0, Some(value.1))
    }
}

impl From<(NotNan<f32>, Option<NotNan<f32>>)> for FloatRange {
    fn from(value: (NotNan<f32>, Option<NotNan<f32>>)) -> Self {
        FloatRange::new(Some(value.0), value.1)
    }
}

impl From<(Option<NotNan<f32>>, Option<NotNan<f32>>)> for FloatRange {
    fn from(value: (Option<NotNan<f32>>, Option<NotNan<f32>>)) -> Self {
        FloatRange::new(value.0, value.1)
    }
}

impl From<Range<NotNan<f32>>> for FloatRange {
    fn from(value: Range<NotNan<f32>>) -> Self {
        FloatRange::new_min_max(value.start, value.end)
    }
}

impl From<Range<Option<NotNan<f32>>>> for FloatRange {
    fn from(value: Range<Option<NotNan<f32>>>) -> Self {
        FloatRange::new(value.start, value.end)
    }
}
