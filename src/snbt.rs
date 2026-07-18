use crate::types::{Double, Float};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

pub type SNBTCompound = BTreeMap<String, SNBT>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SNBT {
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(Float),
    Double(Double),
    String(String),
    List(Vec<Self>),
    Compound(SNBTCompound),
    ByteArray(Vec<i8>),
    IntegerArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl SNBT {
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

        if is_first_char && char.is_ascii_digit() && char != '-' && char != '.' && char != '+' {
            return false;
        }

        if !(char.is_ascii_alphanumeric()
            && char != '_'
            && char != '-'
            && char != '.'
            && char != '+')
        {
            return false;
        }
    }

    true
}

pub(crate) fn fmt_snbt_compound(
    f: &mut Formatter<'_>,
    compound: &SNBTCompound,
) -> std::fmt::Result {
    f.write_str("{")?;

    for (i, (key, value)) in compound.iter().enumerate() {
        if i > 0 {
            f.write_str(", ")?;
        }

        let should_quote = !is_valid_unquoted_snbt_compound_key(key);

        if should_quote {
            f.write_str("\"")?;
        }

        key.fmt(f)?;

        if should_quote {
            f.write_str("\"")?;
        }

        write!(f, ": {}", value)?;
    }

    f.write_str("}")
}

impl Display for SNBT {
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
                    f.write_str("\"")?;
                }

                string.escape_default().fmt(f)?;

                if requires_quotes {
                    f.write_str("\"")?;
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
            Self::Compound(compound) => fmt_snbt_compound(f, compound),
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

impl From<i8> for SNBT {
    fn from(value: i8) -> Self {
        Self::Byte(value)
    }
}

impl From<i16> for SNBT {
    fn from(value: i16) -> Self {
        Self::Short(value)
    }
}

impl From<i32> for SNBT {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<i64> for SNBT {
    fn from(value: i64) -> Self {
        Self::Long(value)
    }
}

impl From<Float> for SNBT {
    fn from(value: Float) -> Self {
        Self::Float(value)
    }
}

impl From<Double> for SNBT {
    fn from(value: Double) -> Self {
        Self::Double(value)
    }
}

impl From<String> for SNBT {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<Self>> for SNBT {
    fn from(value: Vec<Self>) -> Self {
        Self::List(value)
    }
}

impl From<BTreeMap<String, Self>> for SNBT {
    fn from(m: BTreeMap<String, Self>) -> Self {
        Self::Compound(m)
    }
}

impl From<Vec<i8>> for SNBT {
    fn from(value: Vec<i8>) -> Self {
        Self::ByteArray(value)
    }
}

impl From<Vec<i32>> for SNBT {
    fn from(value: Vec<i32>) -> Self {
        Self::IntegerArray(value)
    }
}

impl From<Vec<i64>> for SNBT {
    fn from(value: Vec<i64>) -> Self {
        Self::LongArray(value)
    }
}
