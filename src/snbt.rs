use crate::has_macro::HasMacro;
use crate::macroable::Macroable;
use crate::nbt_path::SNBTCompound;
use crate::types::{Double, Float};
use minecraft_command_types_procedural_macros::HasMacro;
use ordered_float::NotNan;
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::{Serialize, Serializer, de};
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::fmt::{Display, Write};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct SNBTString(pub bool, pub String);

impl HasMacro for SNBTString {
    fn has_macro(&self) -> bool {
        self.0
    }

    fn has_macro_conflict(&self) -> bool {
        if self.0 { false } else { self.1.contains("$(") }
    }
}

impl Serialize for SNBTString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self(false, value) => serializer.serialize_str(value),
            Self(true, name) => {
                let formatted = format!("$({})", name);
                serializer.serialize_str(&formatted)
            }
        }
    }
}

impl Display for SNBTString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.1.fmt(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum SNBT {
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(Float),
    Double(Double),
    String(SNBTString),
    List(Vec<Macroable<Self>>),
    Compound(SNBTCompound),
    ByteArray(Vec<Macroable<i8>>),
    IntegerArray(Vec<Macroable<i32>>),
    LongArray(Vec<Macroable<i64>>),
}

impl SNBT {
    #[inline]
    #[must_use]
    pub const fn macroable_byte(value: i8) -> Macroable<Self> {
        Macroable::Regular(Self::Byte(value))
    }

    #[inline]
    #[must_use]
    pub const fn macroable_short(value: i16) -> Macroable<Self> {
        Macroable::Regular(Self::Short(value))
    }

    #[inline]
    #[must_use]
    pub const fn macroable_integer(value: i32) -> Macroable<Self> {
        Macroable::Regular(Self::Integer(value))
    }

    #[inline]
    #[must_use]
    pub const fn macroable_long(value: i64) -> Macroable<Self> {
        Macroable::Regular(Self::Long(value))
    }

    #[inline]
    #[must_use]
    pub fn float<T: Into<Float>>(value: T) -> Self {
        Self::Float(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_float<T: Into<Float>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::float(value))
    }

    #[inline]
    #[must_use]
    pub fn double<T: Into<Double>>(value: T) -> Self {
        Self::Double(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_double<T: Into<Double>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::double(value))
    }

    #[inline]
    #[must_use]
    pub fn empty_list() -> Self {
        Self::list(Vec::<Self>::new())
    }

    #[inline]
    #[must_use]
    pub fn list<T: Into<Macroable<Self>>>(values: Vec<T>) -> Self {
        Self::List(values.into_iter().map(Into::into).collect())
    }

    #[inline]
    #[must_use]
    pub fn macroable_list<T: Into<Macroable<Self>>>(values: Vec<T>) -> Macroable<Self> {
        Macroable::Regular(Self::list(values))
    }

    #[inline]
    #[must_use]
    pub fn empty_compound() -> Self {
        Self::compound(BTreeMap::<_, Self>::new())
    }

    #[inline]
    #[must_use]
    pub fn compound<V: Into<Macroable<Self>>>(values: BTreeMap<SNBTString, V>) -> Self {
        Self::Compound(values.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    #[inline]
    #[must_use]
    pub fn macroable_compound<V: Into<Macroable<Self>>>(
        values: BTreeMap<SNBTString, V>,
    ) -> Macroable<Self> {
        Macroable::Regular(Self::compound(values))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn string(string: impl ToString) -> Self {
        Self::String(SNBTString(false, string.to_string()))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn macroable_string(string: impl ToString) -> Macroable<Self> {
        Macroable::Regular(Self::String(SNBTString(false, string.to_string())))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub const fn snbt_string(string: SNBTString) -> Self {
        Self::String(string)
    }

    #[must_use]
    pub fn get(&self, key: &SNBTString) -> Option<&Macroable<Self>> {
        if let Self::Compound(compound) = self {
            compound.get(key)
        } else {
            None
        }
    }
}

#[must_use]
pub fn is_valid_unquoted_compound_key(key: &str) -> bool {
    key.chars().all(|char| {
        char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.' || char == '+'
    })
}

#[must_use]
pub fn is_valid_unquoted_string(string: &str) -> bool {
    let mut chars = string.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if first.is_ascii_digit() || matches!(first, '-' | '.' | '+') {
        return false;
    }

    if !first.is_ascii_alphanumeric() && !matches!(first, '_' | '-' | '.' | '+') {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '+'))
}

pub(crate) fn fmt_snbt_compound(
    f: &mut Formatter<'_>,
    compound: &SNBTCompound,
) -> std::fmt::Result {
    f.write_str("{")?;

    for (i, (SNBTString(_, key), value)) in compound.iter().enumerate() {
        if i > 0 {
            f.write_str(", ")?;
        }

        let should_quote = !is_valid_unquoted_compound_key(key);

        if should_quote {
            f.write_char('"')?;
        }

        key.fmt(f)?;

        if should_quote {
            f.write_char('"')?;
        }

        write!(f, ": {}", value)?;
    }

    f.write_str("}")
}

impl Display for SNBT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(byte) => write!(f, "{}b", byte),
            Self::Short(short) => write!(f, "{}s", short),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Long(long) => write!(f, "{}l", long),
            Self::Float(float) => write!(f, "{}f", float),
            Self::Double(double) => write!(f, "{}d", double),
            Self::String(string) => {
                let string = &string.1;

                let should_quote = !is_valid_unquoted_string(string);

                if should_quote {
                    f.write_char('"')?;
                }

                string.escape_default().fmt(f)?;

                if should_quote {
                    f.write_char('"')?;
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

impl Serialize for SNBT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Byte(v) => v.serialize(serializer),
            Self::Short(v) => v.serialize(serializer),
            Self::Integer(v) => v.serialize(serializer),
            Self::Long(v) => v.serialize(serializer),
            Self::Float(value) => serializer.serialize_f32(value.into_inner()),
            Self::Double(value) => serializer.serialize_f64(value.into_inner()),
            Self::String(v) => v.serialize(serializer),
            Self::List(v) => v.serialize(serializer),
            Self::Compound(v) => v.serialize(serializer),
            Self::ByteArray(v) => v.serialize(serializer),
            Self::IntegerArray(v) => v.serialize(serializer),
            Self::LongArray(v) => v.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for SNBT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SNBTVisitor)
    }
}

struct SNBTVisitor;

impl<'de> Visitor<'de> for SNBTVisitor {
    type Value = SNBT;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("any valid SNBT value")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(SNBT::Long(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        i64::try_from(value).map_or_else(
            |_| Err(E::custom(format!("u64 out of range for i64: {}", value))),
            |value| Ok(SNBT::Long(value)),
        )
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NotNan::new(value)
            .map(SNBT::Double)
            .map_err(|_| E::custom("f64 value was NaN"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(SNBT::String(SNBTString(false, value.to_owned())))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::new();

        while let Some(element) = seq.next_element()? {
            list.push(element);
        }

        Ok(SNBT::List(list))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut compound = SNBTCompound::new();
        while let Some((key, value)) = map.next_entry()? {
            compound.insert(SNBTString(false, key), value);
        }
        Ok(SNBT::Compound(compound))
    }
}

impl From<i8> for SNBT {
    fn from(i: i8) -> Self {
        Self::Byte(i)
    }
}

impl From<i16> for SNBT {
    fn from(i: i16) -> Self {
        Self::Short(i)
    }
}

impl From<i32> for SNBT {
    fn from(i: i32) -> Self {
        Self::Integer(i)
    }
}

impl From<i64> for SNBT {
    fn from(i: i64) -> Self {
        Self::Long(i)
    }
}

impl From<Float> for SNBT {
    fn from(f: Float) -> Self {
        Self::Float(f)
    }
}

impl From<Double> for SNBT {
    fn from(f: Double) -> Self {
        Self::Double(f)
    }
}

impl From<String> for SNBT {
    fn from(s: String) -> Self {
        Self::String(SNBTString(false, s))
    }
}

impl From<Vec<Macroable<Self>>> for SNBT {
    fn from(v: Vec<Macroable<Self>>) -> Self {
        Self::List(v)
    }
}

impl From<BTreeMap<SNBTString, Macroable<Self>>> for SNBT {
    fn from(m: BTreeMap<SNBTString, Macroable<Self>>) -> Self {
        Self::Compound(m)
    }
}

impl From<Vec<i8>> for SNBT {
    fn from(v: Vec<i8>) -> Self {
        Self::ByteArray(v.into_iter().map(Macroable::Regular).collect())
    }
}

impl From<Vec<i32>> for SNBT {
    fn from(v: Vec<i32>) -> Self {
        Self::IntegerArray(v.into_iter().map(Macroable::Regular).collect())
    }
}

impl From<Vec<i64>> for SNBT {
    fn from(v: Vec<i64>) -> Self {
        Self::LongArray(v.into_iter().map(Macroable::Regular).collect())
    }
}
