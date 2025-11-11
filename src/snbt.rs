use ordered_float::NotNan;
#[cfg(feature = "serde")]
use serde::de;
#[cfg(feature = "serde")]
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SNBT {
    Byte(i8),
    Boolean(bool),
    Short(i16),
    Integer(i32),
    Long(i64),
    Float(NotNan<f32>),
    Double(NotNan<f64>),
    String(String),
    List(Vec<SNBT>),
    Compound(BTreeMap<String, SNBT>),
    ByteArray(Vec<i8>),
    IntegerArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl SNBT {
    pub fn list<T: Into<SNBT>>(values: Vec<T>) -> SNBT {
        SNBT::List(values.into_iter().map(Into::into).collect())
    }

    pub fn compound<T: Into<SNBT>>(values: BTreeMap<String, T>) -> SNBT {
        SNBT::Compound(values.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    pub fn get(&self, key: &String) -> Option<&SNBT> {
        if let SNBT::Compound(compound) = self {
            compound.get(key)
        } else {
            None
        }
    }
}

impl Display for SNBT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SNBT::Byte(v) => write!(f, "{}b", v),
            SNBT::Boolean(v) => v.fmt(f),
            SNBT::Short(v) => write!(f, "{}s", v),
            SNBT::Integer(v) => write!(f, "{}", v),
            SNBT::Long(v) => write!(f, "{}l", v),
            SNBT::Float(v) => write!(f, "{}f", v),
            SNBT::Double(v) => write!(f, "{}d", v),
            SNBT::String(s) => {
                let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");

                write!(f, "\"{}\"", escaped)
            }
            SNBT::List(values) => {
                write!(f, "[")?;

                for (i, v) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", v)?;
                }

                write!(f, "]")
            }
            SNBT::Compound(map) => {
                write!(f, "{{")?;

                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}:{}", k, v)?;
                }

                write!(f, "}}")
            }
            SNBT::ByteArray(arr) => {
                write!(f, "[B; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}b", v)?;
                }

                write!(f, "]")
            }
            SNBT::IntegerArray(arr) => {
                write!(f, "[I; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", v)?;
                }

                write!(f, "]")
            }
            SNBT::LongArray(arr) => {
                write!(f, "[L; ")?;

                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}L", v)?;
                }

                write!(f, "]")
            }
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for SNBT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SNBT::Byte(v) => serializer.serialize_i8(*v),
            SNBT::Boolean(v) => serializer.serialize_bool(*v),
            SNBT::Short(v) => serializer.serialize_i16(*v),
            SNBT::Integer(v) => serializer.serialize_i32(*v),
            SNBT::Long(v) => serializer.serialize_i64(*v),
            SNBT::Float(v) => serializer.serialize_f32(**v),
            SNBT::Double(v) => serializer.serialize_f64(**v),
            SNBT::String(v) => serializer.serialize_str(v),
            SNBT::List(v) => v.serialize(serializer),
            SNBT::Compound(v) => v.serialize(serializer),
            SNBT::ByteArray(v) => v.serialize(serializer),
            SNBT::IntegerArray(v) => v.serialize(serializer),
            SNBT::LongArray(v) => v.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SNBT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SNBTVisitor)
    }
}

#[cfg(feature = "serde")]
struct SNBTVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for SNBTVisitor {
    type Value = SNBT;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("any valid SNBT value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(SNBT::Boolean(value))
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
        Ok(SNBT::String(value.to_owned()))
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
        let mut compound = BTreeMap::new();
        while let Some((key, value)) = map.next_entry()? {
            compound.insert(key, value);
        }
        Ok(SNBT::Compound(compound))
    }
}

impl From<i8> for SNBT {
    fn from(i: i8) -> Self {
        SNBT::Byte(i)
    }
}

impl From<bool> for SNBT {
    fn from(b: bool) -> Self {
        SNBT::Boolean(b)
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
        SNBT::String(s)
    }
}

impl From<Vec<SNBT>> for SNBT {
    fn from(v: Vec<SNBT>) -> Self {
        SNBT::List(v)
    }
}

impl From<BTreeMap<String, SNBT>> for SNBT {
    fn from(m: BTreeMap<String, SNBT>) -> Self {
        SNBT::Compound(BTreeMap::from(m))
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

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::NotNan;
    use std::collections::BTreeMap;

    fn nnf32(val: f32) -> NotNan<f32> {
        NotNan::new(val).unwrap()
    }

    fn nnf64(val: f64) -> NotNan<f64> {
        NotNan::new(val).unwrap()
    }

    #[test]
    fn test_get_from_compound() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), SNBT::Integer(123));
        map.insert("key2".to_string(), SNBT::String("hello".to_string()));
        let compound = SNBT::Compound(map);

        assert_eq!(compound.get(&"key1".to_string()), Some(&SNBT::Integer(123)));
        assert_eq!(
            compound.get(&"key2".to_string()),
            Some(&SNBT::String("hello".to_string()))
        );
        assert_eq!(compound.get(&"non_existent_key".to_string()), None);
    }

    #[test]
    fn test_get_from_non_compound() {
        let list = SNBT::List(vec![SNBT::Integer(1)]);
        let integer = SNBT::Integer(42);
        let string = SNBT::String("test".to_string());

        assert_eq!(list.get(&"any_key".to_string()), None);
        assert_eq!(integer.get(&"any_key".to_string()), None);
        assert_eq!(string.get(&"any_key".to_string()), None);
    }

    #[test]
    fn test_from_implementations() {
        assert_eq!(SNBT::from(10i8), SNBT::Byte(10));
        assert_eq!(SNBT::from(true), SNBT::Boolean(true));
        assert_eq!(SNBT::from(1000i16), SNBT::Short(1000));
        assert_eq!(SNBT::from(100000i32), SNBT::Integer(100000));
        assert_eq!(SNBT::from(10000000000i64), SNBT::Long(10000000000));
        assert_eq!(SNBT::from(nnf32(1.23)), SNBT::Float(nnf32(1.23)));
        assert_eq!(SNBT::from(nnf64(4.56)), SNBT::Double(nnf64(4.56)));
        assert_eq!(
            SNBT::from("hello".to_string()),
            SNBT::String("hello".to_string())
        );
        assert_eq!(
            SNBT::from(vec![SNBT::Integer(1)]),
            SNBT::List(vec![SNBT::Integer(1)])
        );
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), SNBT::Integer(1));
        assert_eq!(SNBT::from(map.clone()), SNBT::Compound(map));
        assert_eq!(SNBT::from(vec![1, 2, 3]), SNBT::IntegerArray(vec![1, 2, 3]));
    }

    #[cfg(feature = "serde")]
    mod serde_tests {
        use super::*;
        use serde_json;

        fn assert_roundtrip(snbt: &SNBT, expected_json: &str) {
            let json = serde_json::to_string(snbt).unwrap();
            assert_eq!(json, expected_json);

            let deserialized: SNBT = serde_json::from_str(&json).unwrap();
            assert_eq!(*snbt, deserialized);
        }

        #[test]
        fn test_serde_primitives() {
            assert_roundtrip(&SNBT::Boolean(true), "true");
            assert_roundtrip(&SNBT::Long(9223372036854775807), "9223372036854775807");
            assert_roundtrip(&SNBT::Double(nnf64(-1.5e10)), "-15000000000.0");
            assert_roundtrip(
                &SNBT::String("Hello, World!".to_string()),
                "\"Hello, World!\"",
            );
        }

        #[test]
        fn test_serde_list() {
            let list = SNBT::List(vec![
                SNBT::Long(1),
                SNBT::String("two".to_string()),
                SNBT::Boolean(false),
            ]);
            let json = "[1,\"two\",false]";
            assert_roundtrip(&list, json);
        }

        #[test]
        fn test_serde_compound() {
            let mut map = BTreeMap::new();
            map.insert("name".to_string(), SNBT::String("Test".to_string()));
            map.insert("value".to_string(), SNBT::Long(42));
            let compound = SNBT::Compound(map);
            let json = "{\"name\":\"Test\",\"value\":42}";
            assert_roundtrip(&compound, json);
        }

        #[test]
        fn test_serde_nested() {
            let mut root = BTreeMap::new();
            root.insert("id".to_string(), SNBT::Long(123456789));
            root.insert(
                "data".to_string(),
                SNBT::List(vec![
                    SNBT::Compound({
                        let mut item1 = BTreeMap::new();
                        item1.insert("type".to_string(), SNBT::String("A".to_string()));
                        item1.insert("coords".to_string(), SNBT::list(vec![1i64, 2, 3]));
                        item1
                    }),
                    SNBT::Compound({
                        let mut item2 = BTreeMap::new();
                        item2.insert("type".to_string(), SNBT::String("B".to_string()));
                        item2.insert("active".to_string(), SNBT::Boolean(true));
                        item2
                    }),
                ]),
            );
            let snbt = SNBT::Compound(root);
            let json = r#"{"data":[{"coords":[1,2,3],"type":"A"},{"active":true,"type":"B"}],"id":123456789}"#;
            assert_roundtrip(&snbt, json);
        }

        #[test]
        fn test_deserialize_u64_in_range() {
            let u64_val: u64 = 100;
            let json = format!("{}", u64_val);
            let snbt: SNBT = serde_json::from_str(&json).unwrap();
            assert_eq!(snbt, SNBT::Long(100));
        }

        #[test]
        fn test_deserialize_u64_out_of_range_fails() {
            let u64_val: u64 = i64::MAX as u64 + 1;
            let json = format!("{}", u64_val);
            let result: Result<SNBT, _> = serde_json::from_str(&json);
            assert!(result.is_err());
            let err_msg = result.unwrap_err().to_string();
            assert!(err_msg.contains("u64 out of range for i64"));
        }

        #[test]
        fn test_deserialize_nan_double_fails() {
            let result: Result<SNBT, _> = serde_json::from_str("NaN");
            assert!(result.is_err());
        }
    }
}
