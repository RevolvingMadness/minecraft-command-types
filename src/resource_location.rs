use minecraft_command_types_procedural_macros::HasMacro;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt::{self, Display, Formatter, Write};
use std::str::FromStr;

pub type ResourceLocationPaths = Vec<String>;

pub type ResourceLocationPathsRef<'a> = &'a [String];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
pub struct ResourceLocation {
    pub is_tag: bool,
    pub namespace: Option<String>,
    pub paths: ResourceLocationPaths,
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_tag {
            f.write_str("#")?;
        }

        if let Some(namespace) = &self.namespace
            && *namespace != "minecraft"
        {
            write!(f, "{}:", namespace)?;
        }

        for (i, path) in self.paths.iter().enumerate() {
            if i != 0 {
                f.write_char('/')?;
            }

            path.fmt(f)?;
        }

        Ok(())
    }
}

impl ResourceLocation {
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace.as_deref().unwrap_or("minecraft")
    }
}

#[derive(Debug)]
pub enum ResourceLocationParseError {
    EmptyString,
    InvalidFormat(String),
}

impl Display for ResourceLocationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
