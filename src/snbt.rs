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
        false
    }
}

impl Serialize for SNBTString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SNBTString(false, value) => serializer.serialize_str(value),
            SNBTString(true, name) => {
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
    List(Vec<SNBT>),
    Compound(SNBTCompound),
    ByteArray(Vec<i8>),
    IntegerArray(Vec<i32>),
    LongArray(Vec<i64>),
    Macro(String),
}

impl SNBT {
    #[must_use]
    pub fn list<T: Into<SNBT>>(values: Vec<T>) -> SNBT {
        SNBT::List(values.into_iter().map(Into::into).collect())
    }

    #[must_use]
    pub fn compound<T: Into<SNBT>>(values: BTreeMap<SNBTString, T>) -> SNBT {
        SNBT::Compound(values.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    #[inline]
    #[must_use]
    pub fn string<T: ToString>(s: T) -> SNBT {
        SNBT::String(SNBTString(false, s.to_string()))
    }

    #[must_use]
    pub fn get(&self, key: &SNBTString) -> Option<&SNBT> {
        if let SNBT::Compound(compound) = self {
            compound.get(key)
        } else {
            None
        }
    }
}

impl HasMacro for SNBT {
    fn has_macro(&self) -> bool {
        match self {
            SNBT::Macro(_) => true,
            SNBT::List(list) => list.iter().any(|v| v.has_macro()),
            SNBT::Compound(compound) => compound
                .iter()
                .any(|(SNBTString(has_macro, _), value)| *has_macro || value.has_macro()),
            SNBT::String(SNBTString(has_macro, _)) => *has_macro,
            _ => false,
        }
    }

    fn has_macro_conflict(&self) -> bool {
        match self {
            SNBT::List(values) => values.iter().any(|v| v.has_macro_conflict()),
            SNBT::Compound(compound) => compound.values().any(|v| v.has_macro_conflict()),
            SNBT::String(SNBTString(false, value)) => value.contains("$("),
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
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

impl Display for SNBT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SNBT::Byte(v) => write!(f, "{}b", v),
            SNBT::Short(v) => write!(f, "{}s", v),
            SNBT::Integer(v) => write!(f, "{}", v),
            SNBT::Long(v) => write!(f, "{}l", v),
            SNBT::Float(v) => write!(f, "{}f", v),
            SNBT::Double(v) => write!(f, "{}d", v),
            SNBT::String(SNBTString(_, s)) => {
                write!(f, "\"{}\"", escape(s))
            }
            SNBT::List(values) => {
                f.write_str("[")?;

                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    v.fmt(f)?;
                }

                f.write_str("]")
            }
            SNBT::Compound(map) => fmt_snbt_compound(f, map),
            SNBT::ByteArray(arr) => {
                f.write_str("[B; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    write!(f, "{}b", v)?;
                }

                f.write_str("]")
            }
            SNBT::IntegerArray(arr) => {
                f.write_str("[I; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    v.fmt(f)?;
                }

                f.write_str("]")
            }
            SNBT::LongArray(arr) => {
                f.write_str("[L; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    write!(f, "{}L", v)?;
                }

                f.write_str("]")
            }
            SNBT::Macro(name) => write!(f, "$({})", name),
        }
    }
}

impl Serialize for SNBT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SNBT::Byte(v) => serializer.serialize_i8(*v),
            SNBT::Short(v) => serializer.serialize_i16(*v),
            SNBT::Integer(v) => serializer.serialize_i32(*v),
            SNBT::Long(v) => serializer.serialize_i64(*v),
            SNBT::Float(v) => serializer.serialize_f32(**v),
            SNBT::Double(v) => serializer.serialize_f64(**v),
            SNBT::String(SNBTString(_, v)) => serializer.serialize_str(v),
            SNBT::List(v) => v.serialize(serializer),
            SNBT::Compound(v) => v.serialize(serializer),
            SNBT::ByteArray(v) => v.serialize(serializer),
            SNBT::IntegerArray(v) => v.serialize(serializer),
            SNBT::LongArray(v) => v.serialize(serializer),
            SNBT::Macro(v) => format!("<macro {}>", v).serialize(serializer),
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
        if let Ok(v) = i64::try_from(value) {
            Ok(SNBT::Long(v))
        } else {
            Err(E::custom(format!("u64 out of range for i64: {}", value)))
        }
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
        SNBT::Byte(i)
    }
}

impl From<i16> for SNBT {
    fn from(i: i16) -> Self {
        SNBT::Short(i)
    }
}

impl From<i32> for SNBT {
    fn from(i: i32) -> Self {
        SNBT::Integer(i)
    }
}

impl From<i64> for SNBT {
    fn from(i: i64) -> Self {
        SNBT::Long(i)
    }
}

impl From<NotNan<f32>> for SNBT {
    fn from(f: NotNan<f32>) -> Self {
        SNBT::Float(f)
    }
}

impl From<NotNan<f64>> for SNBT {
    fn from(f: NotNan<f64>) -> Self {
        SNBT::Double(f)
    }
}

impl From<String> for SNBT {
    fn from(s: String) -> Self {
        SNBT::String(SNBTString(false, s))
    }
}

impl From<Vec<SNBT>> for SNBT {
    fn from(v: Vec<SNBT>) -> Self {
        SNBT::List(v)
    }
}

impl From<BTreeMap<SNBTString, SNBT>> for SNBT {
    fn from(m: BTreeMap<SNBTString, SNBT>) -> Self {
        SNBT::Compound(m)
    }
}

impl From<Vec<i8>> for SNBT {
    fn from(v: Vec<i8>) -> Self {
        SNBT::ByteArray(v)
    }
}

impl From<Vec<i32>> for SNBT {
    fn from(v: Vec<i32>) -> Self {
        SNBT::IntegerArray(v)
    }
}

impl From<Vec<i64>> for SNBT {
    fn from(v: Vec<i64>) -> Self {
        SNBT::LongArray(v)
    }
}
