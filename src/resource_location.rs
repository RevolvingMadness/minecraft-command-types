use itertools::Itertools;
use minecraft_command_types_procedural_macros::HasMacro;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt::{Display, Formatter};
use std::iter::once;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct ResourceLocation {
    pub is_tag: bool,
    namespace: Option<String>,
    pub paths: Vec<String>,
}

impl ResourceLocation {
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace.as_deref().unwrap_or("minecraft")
    }

    #[inline]
    #[must_use]
    pub fn new<N: ToString, P: ToString>(
        is_tag: bool,
        namespace: Option<N>,
        paths: impl IntoIterator<Item = P>,
    ) -> Self {
        Self {
            is_tag,
            namespace: namespace.map(|namespace| namespace.to_string()),
            paths: paths.into_iter().map(|path| path.to_string()).collect(),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_namespace_paths<P: ToString>(
        namespace: &str,
        paths: impl IntoIterator<Item = P>,
    ) -> Self {
        Self::new(false, Some(namespace), paths)
    }

    #[inline]
    #[must_use]
    pub fn new_namespace_path<P: ToString>(namespace: &str, path: P) -> Self {
        Self::new_namespace_paths(namespace, once(path))
    }

    #[inline]
    #[must_use]
    pub fn new_paths<N: ToString, P: ToString>(paths: impl IntoIterator<Item = P>) -> Self {
        Self::new::<N, _>(false, None, paths)
    }

    #[inline]
    #[must_use]
    pub fn new_path<N: ToString, P: ToString>(path: P) -> Self {
        Self::new_paths::<N, _>(once(path))
    }

    #[must_use]
    pub fn paths_string(&self) -> String {
        self.paths.iter().join("/")
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_tag {
            f.write_str("#")?;
        }

        if let Some(namespace) = &self.namespace
            && *namespace != "minecraft"
        {
            write!(f, "{}:", namespace)?;
        }

        self.paths.iter().join("/").fmt(f)
    }
}

#[derive(Debug)]
pub enum ResourceLocationParseError {
    EmptyString,
    InvalidFormat(String),
}

impl Display for ResourceLocationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyString => f.write_str("Resource location string cannot be empty"),
            Self::InvalidFormat(msg) => {
                write!(f, "Invalid resource location format: {}", msg)
            }
        }
    }
}

impl std::error::Error for ResourceLocationParseError {}

impl FromStr for ResourceLocation {
    type Err = ResourceLocationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ResourceLocationParseError::EmptyString);
        }

        let mut remaining = s;
        let mut is_tag = false;

        if remaining.starts_with('#') {
            is_tag = true;
            remaining = &remaining[1..];
        }

        let parts: Vec<&str> = remaining.split(':').collect();

        let (namespace_raw, path_raw) = match parts.len() {
            1 => (None, parts[0]),
            2 => {
                if parts[0].is_empty() {
                    return Err(ResourceLocationParseError::InvalidFormat(
                        "Namespace component cannot be empty".to_string(),
                    ));
                }
                (Some(parts[0]), parts[1])
            }
            _ => {
                return Err(ResourceLocationParseError::InvalidFormat(
                    "Too many ':' separators".to_string(),
                ));
            }
        };

        if path_raw.is_empty() {
            return Err(ResourceLocationParseError::InvalidFormat(
                "Path component cannot be empty".to_string(),
            ));
        }

        let paths: Vec<String> = path_raw.split('/').map(ToString::to_string).collect();

        let namespace = namespace_raw.map(ToString::to_string);

        Ok(Self {
            is_tag,
            namespace,
            paths,
        })
    }
}

impl Serialize for ResourceLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

struct ResourceLocationVisitor;

impl Visitor<'_> for ResourceLocationVisitor {
    type Value = ResourceLocation;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a Minecraft resource location (e.g., 'minecraft:stone', 'stone', or '#forge:ingots/iron')")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse()
            .map_err(|e| E::custom(format!("failed to parse resource location: {}", e)))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for ResourceLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ResourceLocationVisitor)
    }
}
