use crate::has_macro::HasMacro;
use crate::nbt_path::SNBTCompound;
use ordered_float::NotNan;
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::{Serialize, Serializer, de};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fmt::Formatter;

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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum SNBT {
    Byte(i8),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(NotNan<f32>),
    Double(NotNan<f64>),
    String(SNBTString),
    List(Vec<Self>),
    Compound(SNBTCompound),
    ByteArray(Vec<i8>),
    IntegerArray(Vec<i32>),
    LongArray(Vec<i64>),
    Macro(String),
}

impl SNBT {
    #[must_use]
    pub fn list<T: Into<Self>>(values: Vec<T>) -> Self {
        Self::List(values.into_iter().map(Into::into).collect())
    }

    #[must_use]
    pub fn compound<T: Into<Self>>(values: BTreeMap<SNBTString, T>) -> Self {
        Self::Compound(values.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    #[inline]
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn string(string: impl ToString) -> Self {
        Self::String(SNBTString(false, string.to_string()))
    }

    #[must_use]
    pub fn get(&self, key: &SNBTString) -> Option<&Self> {
        if let Self::Compound(compound) = self {
            compound.get(key)
        } else {
            None
        }
    }
}

impl HasMacro for SNBT {
    fn has_macro(&self) -> bool {
        match self {
            Self::Macro(_) => true,
            Self::List(list) => list.iter().any(HasMacro::has_macro),
            Self::Compound(compound) => compound
                .iter()
                .any(|(SNBTString(has_macro, _), value)| *has_macro || value.has_macro()),
            Self::String(SNBTString(has_macro, _)) => *has_macro,
            _ => false,
        }
    }

    fn has_macro_conflict(&self) -> bool {
        fn fun_name(v: &SNBT) -> bool {
            v.has_macro_conflict()
        }
        match self {
            Self::List(values) => values.iter().any(HasMacro::has_macro_conflict),
            Self::Compound(compound) => compound.values().any(fun_name),
            Self::String(SNBTString(false, value)) => value.contains("$("),
            _ => false,
        }
    }
}

pub fn fmt_snbt_compound(f: &mut Formatter<'_>, compound: &SNBTCompound) -> std::fmt::Result {
    f.write_str("{")?;

    for (i, (SNBTString(_, k), v)) in compound.iter().enumerate() {
        if i > 0 {
            f.write_str(", ")?;
        }

        write!(f, "\"{}\":{}", escape(k), v)?;
    }

    f.write_str("}")
}

#[inline]
#[must_use]
fn escape(input: &str) -> String {
    input.chars().flat_map(char::escape_default).collect()
}

impl Display for SNBT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(v) => write!(f, "{}b", v),
            Self::Short(v) => write!(f, "{}s", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::Long(v) => write!(f, "{}l", v),
            Self::Float(v) => write!(f, "{}f", v),
            Self::Double(v) => write!(f, "{}d", v),
            Self::String(SNBTString(_, s)) => {
                write!(f, "\"{}\"", escape(s))
            }
            Self::List(values) => {
                f.write_str("[")?;

                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    v.fmt(f)?;
                }

                f.write_str("]")
            }
            Self::Compound(map) => fmt_snbt_compound(f, map),
            Self::ByteArray(arr) => {
                f.write_str("[B; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    write!(f, "{}b", v)?;
                }

                f.write_str("]")
            }
            Self::IntegerArray(arr) => {
                f.write_str("[I; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    v.fmt(f)?;
                }

                f.write_str("]")
            }
            Self::LongArray(arr) => {
                f.write_str("[L; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    write!(f, "{}L", v)?;
                }

                f.write_str("]")
            }
            Self::Macro(name) => write!(f, "$({})", name),
        }
    }
}

impl Serialize for SNBT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Byte(v) => serializer.serialize_i8(*v),
            Self::Short(v) => serializer.serialize_i16(*v),
            Self::Integer(v) => serializer.serialize_i32(*v),
            Self::Long(v) => serializer.serialize_i64(*v),
            Self::Float(v) => serializer.serialize_f32(**v),
            Self::Double(v) => serializer.serialize_f64(**v),
            Self::String(SNBTString(_, v)) => serializer.serialize_str(v),
            Self::List(v) => v.serialize(serializer),
            Self::Compound(v) => v.serialize(serializer),
            Self::ByteArray(v) => v.serialize(serializer),
            Self::IntegerArray(v) => v.serialize(serializer),
            Self::LongArray(v) => v.serialize(serializer),
            Self::Macro(v) => format!("<macro {}>", v).serialize(serializer),
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
            |v| Ok(SNBT::Long(v)),
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
        while let Some(elem) = seq.next_element()? {
            list.push(elem);
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

impl From<NotNan<f32>> for SNBT {
    fn from(f: NotNan<f32>) -> Self {
        Self::Float(f)
    }
}

impl From<NotNan<f64>> for SNBT {
    fn from(f: NotNan<f64>) -> Self {
        Self::Double(f)
    }
}

impl From<String> for SNBT {
    fn from(s: String) -> Self {
        Self::String(SNBTString(false, s))
    }
}

impl From<Vec<Self>> for SNBT {
    fn from(v: Vec<Self>) -> Self {
        Self::List(v)
    }
}

impl From<BTreeMap<SNBTString, Self>> for SNBT {
    fn from(m: BTreeMap<SNBTString, Self>) -> Self {
        Self::Compound(m)
    }
}

impl From<Vec<i8>> for SNBT {
    fn from(v: Vec<i8>) -> Self {
        Self::ByteArray(v)
    }
}

impl From<Vec<i32>> for SNBT {
    fn from(v: Vec<i32>) -> Self {
        Self::IntegerArray(v)
    }
}

impl From<Vec<i64>> for SNBT {
    fn from(v: Vec<i64>) -> Self {
        Self::LongArray(v)
    }
}
