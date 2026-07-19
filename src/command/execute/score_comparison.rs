use std::fmt::{self, Display, Formatter};

use strum::Display;

use crate::{player_score::PlayerScore, range::IntegerRange};

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScoreComparisonOperator {
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = "<=")]
    LessThanOrEqualTo,
    #[strum(serialize = "=")]
    EqualTo,
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = ">=")]
    GreaterThanOrEqualTo,
}

impl ScoreComparisonOperator {
    #[must_use]
    pub const fn into_range(self, value: i32) -> IntegerRange {
        match self {
            Self::LessThan => IntegerRange::new_upper(value - 1),
            Self::LessThanOrEqualTo => IntegerRange::new_upper(value),
            Self::EqualTo => IntegerRange::new_single(value),
            Self::GreaterThan => IntegerRange::new_lower(value + 1),
            Self::GreaterThanOrEqualTo => IntegerRange::new_lower(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreComparison {
    Range(IntegerRange),
    Score(ScoreComparisonOperator, PlayerScore),
}

impl Display for ScoreComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range(range) => write!(f, "matches {}", range),
            Self::Score(operator, right) => {
                write!(f, "{} {}", operator, right)
            }
        }
    }
}
