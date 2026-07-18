use crate::{
    command::enums::{gamemode::Gamemode, sort::Sort},
    range::{FloatRange, IntegerRange},
    resource_location::ResourceLocation,
    snbt::SNBT,
    types::Float,
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntitySelectorVariable {
    P,
    R,
    A,
    E,
    S,
    N,
}

impl FromStr for EntitySelectorVariable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "a" => Self::A,
            "E" | "e" => Self::E,
            "N" | "n" => Self::N,
            "P" | "p" => Self::P,
            "R" | "r" => Self::R,
            "S" | "s" => Self::S,

            _ => return Err(()),
        })
    }
}

impl Display for EntitySelectorVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::P => "p",
            Self::R => "r",
            Self::A => "a",
            Self::E => "e",
            Self::S => "s",
            Self::N => "n",
        }
        .fmt(f)
    }
}

fn fmt_hash_map<K: Display, V: Display>(
    f: &mut Formatter<'_>,
    input: &BTreeMap<K, V>,
) -> std::fmt::Result {
    f.write_str("{")?;
    let mut first = true;

    for (k, v) in input {
        if !first {
            f.write_str(", ")?;
        }

        first = false;

        write!(f, "{}={}", k, v)?;
    }

    f.write_str("}")
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdvancementChoiceType {
    Boolean(bool),
    Criterion(BTreeMap<String, bool>),
}

impl From<bool> for AdvancementChoiceType {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<BTreeMap<String, bool>> for AdvancementChoiceType {
    fn from(value: BTreeMap<String, bool>) -> Self {
        Self::Criterion(value)
    }
}

impl Display for AdvancementChoiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean(bool) => bool.fmt(f),
            Self::Criterion(map) => fmt_hash_map(f, map),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntitySelectorOption {
    X(Float),
    Y(Float),
    Z(Float),
    Distance(FloatRange),
    DistanceX(Float),
    DistanceY(Float),
    DistanceZ(Float),
    XRotation(FloatRange),
    YRotation(FloatRange),
    Scores(BTreeMap<String, IntegerRange>),
    Tag(bool, String),
    Team(bool, String),
    Name(bool, String),
    Type(bool, ResourceLocation),
    Predicate(bool, ResourceLocation),
    Nbt(bool, SNBT),
    Gamemode(bool, Gamemode),
    Level(IntegerRange),
    Advancements(BTreeMap<ResourceLocation, AdvancementChoiceType>),
    Limit(i32),
    Sort(Sort),
}

impl EntitySelectorOption {
    #[must_use]
    pub const fn can_be_repeated(&self) -> bool {
        matches!(
            self,
            Self::Tag(..)
                | Self::Team(true, _)
                | Self::Name(true, _)
                | Self::Type(true, _)
                | Self::Predicate(..)
                | Self::Nbt(..)
                | Self::Gamemode(true, _)
        )
    }
}

macro_rules! write_entity_selector_option {
    ($f:expr, $key:literal, $inverted:expr, $value:expr) => {{
        write!($f, "{}=", $key)?;

        if $inverted {
            $f.write_str("!")?;
        }

        $value.fmt($f)
    }};
    ($f:expr, $key:literal, $value:expr) => {
        write!($f, "{}={}", $key, $value)
    };
}

impl Display for EntitySelectorOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::X(x) => write_entity_selector_option!(f, "x", x),
            Self::Y(y) => write_entity_selector_option!(f, "y", y),
            Self::Z(z) => write_entity_selector_option!(f, "z", z),
            Self::Distance(d) => write_entity_selector_option!(f, "distance", d),
            Self::DistanceX(dx) => write_entity_selector_option!(f, "dx", dx),
            Self::DistanceY(dy) => write_entity_selector_option!(f, "dy", dy),
            Self::DistanceZ(dz) => write_entity_selector_option!(f, "dz", dz),
            Self::XRotation(rot) => {
                write_entity_selector_option!(f, "x_rotation", rot)
            }
            Self::YRotation(rot) => {
                write_entity_selector_option!(f, "y_rotation", rot)
            }
            Self::Level(level) => write_entity_selector_option!(f, "level", level),
            Self::Limit(limit) => write_entity_selector_option!(f, "limit", limit),
            Self::Sort(sort) => write_entity_selector_option!(f, "sort", sort),

            Self::Tag(inv, val) => {
                write_entity_selector_option!(f, "tag", *inv, val)
            }
            Self::Team(inv, val) => {
                write_entity_selector_option!(f, "team", *inv, val)
            }
            Self::Name(inv, val) => {
                write_entity_selector_option!(f, "name", *inv, val)
            }
            Self::Type(inv, val) => {
                write_entity_selector_option!(f, "type", *inv, val)
            }
            Self::Predicate(inv, val) => {
                write_entity_selector_option!(f, "predicate", *inv, val)
            }
            Self::Nbt(inv, val) => {
                write_entity_selector_option!(f, "nbt", *inv, val)
            }
            Self::Gamemode(inv, val) => {
                write_entity_selector_option!(f, "gamemode", *inv, val)
            }

            Self::Scores(scores) => {
                f.write_str("scores=")?;

                fmt_hash_map(f, scores)
            }
            Self::Advancements(advancements) => {
                f.write_str("advancements=")?;

                fmt_hash_map(f, advancements)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntitySelector {
    Variable(EntitySelectorVariable, Vec<EntitySelectorOption>),
    Name(String),
}

impl EntitySelector {
    pub const A: Self = Self::Variable(EntitySelectorVariable::A, Vec::new());
    pub const E: Self = Self::Variable(EntitySelectorVariable::E, Vec::new());
    pub const N: Self = Self::Variable(EntitySelectorVariable::N, Vec::new());
    pub const P: Self = Self::Variable(EntitySelectorVariable::P, Vec::new());
    pub const R: Self = Self::Variable(EntitySelectorVariable::R, Vec::new());
    pub const S: Self = Self::Variable(EntitySelectorVariable::S, Vec::new());

    #[inline]
    #[must_use]
    pub const fn new(variable: EntitySelectorVariable, options: Vec<EntitySelectorOption>) -> Self {
        Self::Variable(variable, options)
    }

    #[inline]
    #[must_use]
    pub const fn p(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::P, options)
    }

    #[inline]
    #[must_use]
    pub const fn r(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::R, options)
    }

    #[inline]
    #[must_use]
    pub const fn a(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::A, options)
    }

    #[inline]
    #[must_use]
    pub const fn e(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::E, options)
    }

    #[inline]
    #[must_use]
    pub const fn s(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::S, options)
    }

    #[inline]
    #[must_use]
    pub const fn n(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::N, options)
    }
}

impl Default for EntitySelector {
    fn default() -> Self {
        Self::S
    }
}

impl Display for EntitySelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(variable, options) => {
                write!(f, "@{}", variable)?;

                if !options.is_empty() {
                    f.write_str("[")?;
                    let mut first = true;

                    for option in options {
                        if !first {
                            f.write_str(", ")?;
                        }

                        option.fmt(f)?;

                        first = false;
                    }

                    f.write_str("]")?;
                }

                Ok(())
            }
            Self::Name(name) => name.fmt(f),
        }
    }
}

impl From<EntitySelectorVariable> for EntitySelector {
    fn from(value: EntitySelectorVariable) -> Self {
        Self::Variable(value, Vec::new())
    }
}

impl From<String> for EntitySelector {
    fn from(value: String) -> Self {
        Self::Name(value)
    }
}
