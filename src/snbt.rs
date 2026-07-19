use crate::types::{Double, Float};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

pub type SnbtCompound = BTreeMap<String, Snbt>;

pub(crate) trait SnbtCompoundExt {
    fn display_as_compound(&self) -> SnbtCompoundDisplay<'_>;
}

impl SnbtCompoundExt for SnbtCompound {
    fn display_as_compound(&self) -> SnbtCompoundDisplay<'_> {
        SnbtCompoundDisplay { this: self }
    }
}

pub(crate) struct SnbtCompoundDisplay<'this> {
    this: &'this SnbtCompound,
}

impl Display for SnbtCompoundDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (key, value)) in self.this.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            let should_quote = !is_valid_unquoted_snbt_compound_key(key);

            if should_quote {
                write!(f, "\"")?;
            }

            key.fmt(f)?;

            if should_quote {
                write!(f, "\"")?;
            }

            write!(f, ": {}", value)?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Snbt {
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(Float),
    Double(Double),
    String(String),
    List(Vec<Self>),
    Compound(SnbtCompound),
    ByteArray(Vec<i8>),
    IntegerArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Snbt {
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Self> {
        let Self::Compound(compound) = self else {
            return None;
        };

        compound.get(key)
    }
}

#[must_use]
pub fn is_valid_unquoted_snbt_compound_key(key: &str) -> bool {
    key.chars().all(|char| {
        char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.' || char == '+'
    })
}

#[must_use]
pub fn is_valid_unquoted_snbt_string(string: &str) -> bool {
    for (i, char) in string.chars().enumerate() {
        let is_first_char = i == 0;

        if is_first_char && (char.is_ascii_digit() || char == '-' || char == '.' || char == '+') {
            return false;
        }

        if !char.is_ascii_alphanumeric() && char != '_' && char != '-' && char != '.' && char != '+'
        {
            return false;
        }
    }

    true
}

impl Display for Snbt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte(byte) => write!(f, "{}b", byte),
            Self::Short(short) => write!(f, "{}s", short),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Long(long) => write!(f, "{}l", long),
            Self::Float(float) => write!(f, "{}f", float),
            Self::Double(double) => write!(f, "{}d", double),
            Self::String(string) => {
                let requires_quotes = !is_valid_unquoted_snbt_string(string);

                if requires_quotes {
                    write!(f, "\"")?;
                }

                string.escape_default().fmt(f)?;

                if requires_quotes {
                    write!(f, "\"")?;
                }

                Ok(())
            }
            Self::List(list) => {
                f.write_str("[")?;

                for (index, snbt) in list.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }

                    snbt.fmt(f)?;
                }

                f.write_str("]")
            }
            Self::Compound(compound) => write!(f, "{}", (*compound).display_as_compound()),
            Self::ByteArray(byte_array) => {
                f.write_str("[B; ")?;

                for (i, byte) in byte_array.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    write!(f, "{}b", byte)?;
                }

                f.write_str("]")
            }
            Self::IntegerArray(integer_array) => {
                f.write_str("[I; ")?;

                for (i, integer) in integer_array.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    integer.fmt(f)?;
                }

                f.write_str("]")
            }
            Self::LongArray(long_array) => {
                f.write_str("[L; ")?;

                for (i, long) in long_array.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    write!(f, "{}L", long)?;
                }

                f.write_str("]")
            }
        }
    }
}

impl From<i8> for Snbt {
    fn from(value: i8) -> Self {
        Self::Byte(value)
    }
}

impl From<i16> for Snbt {
    fn from(value: i16) -> Self {
        Self::Short(value)
    }
}

impl From<i32> for Snbt {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<i64> for Snbt {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<Float> for Snbt {
    fn from(value: Float) -> Self {
        Self::Float(value)
    }
}

impl From<Double> for Snbt {
    fn from(value: Double) -> Self {
        Self::Double(value)
    }
}

impl From<String> for Snbt {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<Self>> for Snbt {
    fn from(value: Vec<Self>) -> Self {
        Self::List(value)
    }
}

impl From<BTreeMap<String, Self>> for Snbt {
    fn from(m: BTreeMap<String, Self>) -> Self {
        Self::Compound(m)
    }
}

impl From<Vec<i8>> for Snbt {
    fn from(value: Vec<i8>) -> Self {
        Self::ByteArray(value)
    }
}

impl From<Vec<i32>> for Snbt {
    fn from(value: Vec<i32>) -> Self {
        Self::IntegerArray(value)
    }
}

impl From<Vec<i64>> for Snbt {
    fn from(value: Vec<i64>) -> Self {
        Self::LongArray(value)
    }
}
