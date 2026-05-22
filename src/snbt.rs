use crate::has_macro::HasMacro;
use crate::macroable::Macroable;
use crate::nbt_path::SNBTCompound;
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
    Byte(Macroable<i8>),
    Short(Macroable<i16>),
    Integer(Macroable<i32>),
    Long(Macroable<i64>),
    Float(Macroable<NotNan<f32>>),
    Double(Macroable<NotNan<f64>>),
    String(Macroable<SNBTString>),
    List(Macroable<Vec<Macroable<Self>>>),
    Compound(Macroable<SNBTCompound>),
    ByteArray(Macroable<Vec<Macroable<i8>>>),
    IntegerArray(Macroable<Vec<Macroable<i32>>>),
    LongArray(Macroable<Vec<Macroable<i64>>>),
}

impl SNBT {
    #[inline]
    #[must_use]
    pub fn byte<T: Into<Macroable<i8>>>(value: T) -> Self {
        Self::Byte(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_byte<T: Into<Macroable<i8>>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::byte(value))
    }

    #[inline]
    #[must_use]
    pub fn short<T: Into<Macroable<i16>>>(value: T) -> Self {
        Self::Short(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_short<T: Into<Macroable<i16>>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::short(value))
    }

    #[inline]
    #[must_use]
    pub fn integer<T: Into<Macroable<i32>>>(value: T) -> Self {
        Self::Integer(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_integer<T: Into<Macroable<i32>>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::integer(value))
    }

    #[inline]
    #[must_use]
    pub fn long<T: Into<Macroable<i64>>>(value: T) -> Self {
        Self::Long(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_long<T: Into<Macroable<i64>>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::long(value))
    }

    #[inline]
    #[must_use]
    pub fn float<T: Into<Macroable<NotNan<f32>>>>(value: T) -> Self {
        Self::Float(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_float<T: Into<Macroable<NotNan<f32>>>>(value: T) -> Macroable<Self> {
        Macroable::Regular(Self::float(value))
    }

    #[inline]
    #[must_use]
    pub fn double<T: Into<Macroable<NotNan<f64>>>>(value: T) -> Self {
        Self::Double(value.into())
    }

    #[inline]
    #[must_use]
    pub fn macroable_double<T: Into<Macroable<NotNan<f64>>>>(value: T) -> Macroable<Self> {
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
        Self::String(Macroable::Regular(SNBTString(false, string.to_string())))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn macroable_string(string: impl ToString) -> Macroable<Self> {
        Macroable::Regular(Self::String(Macroable::Regular(SNBTString(
            false,
            string.to_string(),
        ))))
    }

    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub const fn snbt_string(string: SNBTString) -> Self {
        Self::String(Macroable::Regular(string))
    }

    #[must_use]
    pub fn get(&self, key: &SNBTString) -> Option<&Macroable<Self>> {
        if let Self::Compound(Macroable::Regular(compound)) = self {
            compound.get(key)
        } else {
            None
        }
    }
}

#[must_use]
pub fn is_valid_unquoted_compound_key(string: &str) -> bool {
    string.chars().all(|char| {
        char.is_ascii_alphanumeric() || char == '_' || char == '-' || char == '.' || char == '+'
    })
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
            Self::String(macroable) => match macroable {
                Macroable::Regular(SNBTString(_, string)) => {
                    write!(f, "\"{}\"", string.escape_default())
                }
                Macroable::Macro(name) => f.write_str(name),
            },
            Self::List(list) => match list {
                Macroable::Regular(list) => {
                    f.write_str("[")?;

                    for (index, snbt) in list.iter().enumerate() {
                        if index > 0 {
                            f.write_str(", ")?;
                        }

                        snbt.fmt(f)?;
                    }

                    f.write_str("]")
                }
                Macroable::Macro(name) => f.write_str(name),
            },
            Self::Compound(compound) => match compound {
                Macroable::Regular(compound) => fmt_snbt_compound(f, compound),
                Macroable::Macro(name) => f.write_str(name),
            },
            Self::ByteArray(byte_array) => match byte_array {
                Macroable::Regular(byte_array) => {
                    f.write_str("[B; ")?;

                    for (i, byte) in byte_array.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }

                        write!(f, "{}b", byte)?;
                    }

                    f.write_str("]")
                }
                Macroable::Macro(name) => f.write_str(name),
            },
            Self::IntegerArray(integer_array) => match integer_array {
                Macroable::Regular(integer_array) => {
                    f.write_str("[I; ")?;

                    for (i, integer) in integer_array.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }

                        integer.fmt(f)?;
                    }

                    f.write_str("]")
                }
                Macroable::Macro(name) => f.write_str(name),
            },
            Self::LongArray(long_array) => match long_array {
                Macroable::Regular(long_array) => {
                    f.write_str("[L; ")?;

                    for (i, long) in long_array.iter().enumerate() {
                        if i > 0 {
                            f.write_str(", ")?;
                        }

                        write!(f, "{}L", long)?;
                    }

                    f.write_str("]")
                }
                Macroable::Macro(name) => f.write_str(name),
            },
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
            Self::Float(macroable) => match macroable {
                Macroable::Regular(value) => serializer.serialize_f32(value.into_inner()),
                Macroable::Macro(name) => serializer.serialize_str(name),
            },
            Self::Double(macroable) => match macroable {
                Macroable::Regular(value) => serializer.serialize_f64(value.into_inner()),
                Macroable::Macro(name) => serializer.serialize_str(name),
            },
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
        Ok(SNBT::Long(Macroable::Regular(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        i64::try_from(value).map_or_else(
            |_| Err(E::custom(format!("u64 out of range for i64: {}", value))),
            |value| Ok(SNBT::Long(Macroable::Regular(value))),
        )
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NotNan::new(value)
            .map(|value| SNBT::Double(Macroable::Regular(value)))
            .map_err(|_| E::custom("f64 value was NaN"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(SNBT::String(Macroable::Regular(SNBTString(
            false,
            value.to_owned(),
        ))))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut list = Vec::new();

        while let Some(element) = seq.next_element()? {
            list.push(element);
        }

        Ok(SNBT::List(Macroable::Regular(list)))
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut compound = SNBTCompound::new();
        while let Some((key, value)) = map.next_entry()? {
            compound.insert(SNBTString(false, key), value);
        }
        Ok(SNBT::Compound(Macroable::Regular(compound)))
    }
}

impl From<i8> for SNBT {
    fn from(i: i8) -> Self {
        Self::Byte(Macroable::Regular(i))
    }
}

impl From<i16> for SNBT {
    fn from(i: i16) -> Self {
        Self::Short(Macroable::Regular(i))
    }
}

impl From<i32> for SNBT {
    fn from(i: i32) -> Self {
        Self::Integer(Macroable::Regular(i))
    }
}

impl From<i64> for SNBT {
    fn from(i: i64) -> Self {
        Self::Long(Macroable::Regular(i))
    }
}

impl From<NotNan<f32>> for SNBT {
    fn from(f: NotNan<f32>) -> Self {
        Self::Float(Macroable::Regular(f))
    }
}

impl From<NotNan<f64>> for SNBT {
    fn from(f: NotNan<f64>) -> Self {
        Self::Double(Macroable::Regular(f))
    }
}

impl From<String> for SNBT {
    fn from(s: String) -> Self {
        Self::String(Macroable::Regular(SNBTString(false, s)))
    }
}

impl From<Vec<Macroable<Self>>> for SNBT {
    fn from(v: Vec<Macroable<Self>>) -> Self {
        Self::List(Macroable::Regular(v))
    }
}

impl From<BTreeMap<SNBTString, Macroable<Self>>> for SNBT {
    fn from(m: BTreeMap<SNBTString, Macroable<Self>>) -> Self {
        Self::Compound(Macroable::Regular(m))
    }
}

impl From<Vec<i8>> for SNBT {
    fn from(v: Vec<i8>) -> Self {
        Self::ByteArray(Macroable::Regular(
            v.into_iter().map(Macroable::Regular).collect(),
        ))
    }
}

impl From<Vec<i32>> for SNBT {
    fn from(v: Vec<i32>) -> Self {
        Self::IntegerArray(Macroable::Regular(
            v.into_iter().map(Macroable::Regular).collect(),
        ))
    }
}

impl From<Vec<i64>> for SNBT {
    fn from(v: Vec<i64>) -> Self {
        Self::LongArray(Macroable::Regular(
            v.into_iter().map(Macroable::Regular).collect(),
        ))
    }
}
