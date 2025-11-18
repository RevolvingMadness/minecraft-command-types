use serde::ser::SerializeTuple;
use serde::{Serialize, Serializer, de};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Format {
    Integer(i32),
    Array(i32, i32),
}

impl Serialize for Format {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Format::Integer(i) => serializer.serialize_i32(*i),

            Format::Array(a, b) => {
                let mut seq = serializer.serialize_tuple(2)?;

                seq.serialize_element(a)?;
                seq.serialize_element(b)?;

                seq.end()
            }
        }
    }
}

struct FormatVisitor;

impl<'de> de::Visitor<'de> for FormatVisitor {
    type Value = Format;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or an array of two integers, e.g., 5 or [1, 2]")
    }

    fn visit_i32<E>(self, i: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Format::Integer(i))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let a: i32 = match seq.next_element()? {
            Some(val) => val,
            None => return Err(de::Error::invalid_length(0, &self)),
        };

        let b: i32 = match seq.next_element()? {
            Some(val) => val,
            None => return Err(de::Error::invalid_length(1, &self)),
        };

        if seq.next_element::<i32>()?.is_some() {
            return Err(de::Error::invalid_length(3, &self));
        }

        Ok(Format::Array(a, b))
    }
}

impl<'de> de::Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(FormatVisitor)
    }
}
