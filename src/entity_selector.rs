use crate::command::enums::gamemode::Gamemode;
use crate::command::enums::sort::Sort;
use crate::range::{FloatRange, IntegerRange};
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum EntitySelectorVariable {
    P,
    R,
    A,
    E,
    S,
    N,
}

impl Display for EntitySelectorVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(bool) => bool.fmt(f),
            Self::Criterion(map) => fmt_hash_map(f, map),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum EntitySelectorOption {
    X(NotNan<f32>),
    Y(NotNan<f32>),
    Z(NotNan<f32>),
    Distance(FloatRange),
    DistanceX(NotNan<f32>),
    DistanceY(NotNan<f32>),
    DistanceZ(NotNan<f32>),
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

        if *$inverted {
            $f.write_str("!")?;
        }

        $value.fmt($f)
    }};
    ($f:expr, $key:literal, $value:expr) => {
        write!($f, "{}={}", $key, $value)
    };
}

impl Display for EntitySelectorOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
                write_entity_selector_option!(f, "tag", inv, val)
            }
            Self::Team(inv, val) => {
                write_entity_selector_option!(f, "team", inv, val)
            }
            Self::Name(inv, val) => {
                write_entity_selector_option!(f, "name", inv, val)
            }
            Self::Type(inv, val) => {
                write_entity_selector_option!(f, "type", inv, val)
            }
            Self::Predicate(inv, val) => {
                write_entity_selector_option!(f, "predicate", inv, val)
            }
            Self::Nbt(inv, val) => {
                write_entity_selector_option!(f, "nbt", inv, val)
            }
            Self::Gamemode(inv, val) => {
                write_entity_selector_option!(f, "gamemode", inv, val)
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum EntitySelector {
    Variable(EntitySelectorVariable, Vec<EntitySelectorOption>),
    Name(String),
}

impl EntitySelector {
    #[must_use]
    pub const fn new(variable: EntitySelectorVariable, options: Vec<EntitySelectorOption>) -> Self {
        Self::Variable(variable, options)
    }

    #[must_use]
    pub const fn p(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::P, options)
    }

    #[must_use]
    pub const fn p_no_options() -> Self {
        Self::p(vec![])
    }

    #[must_use]
    pub const fn r(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::R, options)
    }

    #[must_use]
    pub const fn r_no_options() -> Self {
        Self::r(vec![])
    }

    #[must_use]
    pub const fn a(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::A, options)
    }

    #[must_use]
    pub const fn a_no_options() -> Self {
        Self::a(vec![])
    }

    #[must_use]
    pub const fn e(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::E, options)
    }

    #[must_use]
    pub const fn e_no_options() -> Self {
        Self::e(vec![])
    }

    #[must_use]
    pub const fn s(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::S, options)
    }

    #[must_use]
    pub const fn s_no_options() -> Self {
        Self::s(vec![])
    }

    #[must_use]
    pub const fn n(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::N, options)
    }

    #[must_use]
    pub const fn n_no_options() -> Self {
        Self::n(vec![])
    }
}

impl Default for EntitySelector {
    fn default() -> Self {
        Self::s_no_options()
    }
}

impl Display for EntitySelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[cfg(test)]
mod tests {
    use crate::snbt::SNBTString;

    use super::*;
    use ordered_float::NotNan;
    use std::collections::BTreeMap;

    fn nn(val: f32) -> NotNan<f32> {
        NotNan::new(val).unwrap()
    }

    #[test]
    fn test_format_no_options() {
        assert_eq!(EntitySelector::p_no_options().to_string(), "@p");
        assert_eq!(EntitySelector::r_no_options().to_string(), "@r");
        assert_eq!(EntitySelector::a_no_options().to_string(), "@a");
        assert_eq!(EntitySelector::e_no_options().to_string(), "@e");
        assert_eq!(EntitySelector::s_no_options().to_string(), "@s");
        assert_eq!(EntitySelector::n_no_options().to_string(), "@n");
    }

    #[test]
    fn test_format_simple_options() {
        assert_eq!(
            EntitySelector::p(vec![EntitySelectorOption::X(nn(10.5))]).to_string(),
            "@p[x=10.5]"
        );
        assert_eq!(
            EntitySelector::p(vec![EntitySelectorOption::Y(nn(-5.0))]).to_string(),
            "@p[y=-5]"
        );
        assert_eq!(
            EntitySelector::p(vec![EntitySelectorOption::Z(nn(0.0))]).to_string(),
            "@p[z=0]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::DistanceX(nn(10.0))]).to_string(),
            "@e[dx=10]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Limit(1)]).to_string(),
            "@e[limit=1]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Sort(Sort::Nearest)]).to_string(),
            "@e[sort=nearest]"
        );
    }

    #[test]
    fn test_format_range_options() {
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Distance(
                FloatRange::new_single(nn(5.0))
            )])
            .to_string(),
            "@e[distance=5]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Distance(FloatRange::new_min(
                nn(5.0)
            ))])
            .to_string(),
            "@e[distance=5..]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Distance(FloatRange::new_max(
                nn(10.2)
            ))])
            .to_string(),
            "@e[distance=..10.2]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Distance(
                FloatRange::new_min_max(nn(5.0), nn(10.0))
            )])
            .to_string(),
            "@e[distance=5..10]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Level(IntegerRange::new_single(
                10
            ))])
            .to_string(),
            "@a[level=10]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Level(IntegerRange::new_min(5))])
                .to_string(),
            "@a[level=5..]"
        );
        assert_eq!(
            EntitySelector::p(vec![EntitySelectorOption::XRotation(
                FloatRange::new_min_max(nn(-90.0), nn(90.0))
            )])
            .to_string(),
            "@p[x_rotation=-90..90]"
        );
    }

    #[test]
    fn test_format_invertible_options() {
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Tag(
                false,
                "friendly".to_string()
            )])
            .to_string(),
            "@e[tag=friendly]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Tag(true, "hostile".to_string())])
                .to_string(),
            "@e[tag=!hostile]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Team(false, "blue".to_string())])
                .to_string(),
            "@a[team=blue]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Team(true, "red".to_string())])
                .to_string(),
            "@a[team=!red]"
        );
        assert_eq!(
            EntitySelector::p(vec![EntitySelectorOption::Name(false, "Steve".to_string())])
                .to_string(),
            "@p[name=Steve]"
        );
        assert_eq!(
            EntitySelector::p(vec![EntitySelectorOption::Name(true, "Alex".to_string())])
                .to_string(),
            "@p[name=!Alex]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Type(
                false,
                ResourceLocation::new_namespace_path("minecraft", "pig")
            )])
            .to_string(),
            "@e[type=pig]"
        );
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Type(
                true,
                ResourceLocation::new_namespace_path("minecraft", "zombie")
            )])
            .to_string(),
            "@e[type=!zombie]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Gamemode(
                false,
                Gamemode::Survival
            )])
            .to_string(),
            "@a[gamemode=survival]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Gamemode(
                true,
                Gamemode::Creative
            )])
            .to_string(),
            "@a[gamemode=!creative]"
        );
        let mut compound = BTreeMap::new();
        compound.insert(SNBTString(false, "OnGround".to_string()), SNBT::Byte(1));
        let nbt = SNBT::Compound(compound);
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Nbt(false, nbt)]).to_string(),
            "@e[nbt={OnGround:1b}]"
        );
        let mut compound = BTreeMap::new();
        compound.insert(SNBTString(false, "Air".to_string()), SNBT::Short(300));
        let nbt = SNBT::Compound(compound);
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Nbt(true, nbt)]).to_string(),
            "@e[nbt=!{Air:300s}]"
        );
    }

    #[test]
    fn test_format_map_options() {
        let mut scores = BTreeMap::new();
        scores.insert("kills".to_string(), IntegerRange::new_min(10));
        scores.insert("deaths".to_string(), IntegerRange::new_single(0));
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Scores(scores)]).to_string(),
            "@a[scores={deaths=0, kills=10..}]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Scores(BTreeMap::new())]).to_string(),
            "@a[scores={}]"
        );

        let mut advancements = BTreeMap::new();
        advancements.insert(
            ResourceLocation::new_namespace_path("minecraft", "story/root"),
            AdvancementChoiceType::Boolean(true),
        );
        let mut criteria = BTreeMap::new();
        criteria.insert("has_effect".to_string(), true);
        criteria.insert("missing_effect".to_string(), false);
        advancements.insert(
            ResourceLocation::new_namespace_path("minecraft", "nether/all_potions"),
            AdvancementChoiceType::Criterion(criteria),
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Advancements(advancements)]).to_string(),
            "@a[advancements={nether/all_potions={has_effect=true, missing_effect=false}, story/root=true}]"
        );
        assert_eq!(
            EntitySelector::a(vec![EntitySelectorOption::Advancements(BTreeMap::new())])
                .to_string(),
            "@a[advancements={}]"
        );
    }

    #[test]
    fn test_format_multiple_options() {
        let selector = EntitySelector::e(vec![
            EntitySelectorOption::Type(
                false,
                ResourceLocation::new_namespace_path("minecraft", "creeper"),
            ),
            EntitySelectorOption::Distance(FloatRange::new_max(nn(10.0))),
            EntitySelectorOption::Limit(1),
            EntitySelectorOption::Sort(Sort::Nearest),
        ]);
        assert_eq!(
            selector.to_string(),
            "@e[type=creeper, distance=..10, limit=1, sort=nearest]"
        );
    }

    #[test]
    fn test_format_repeatable_options() {
        let selector = EntitySelector::a(vec![
            EntitySelectorOption::Gamemode(true, Gamemode::Creative),
            EntitySelectorOption::Level(IntegerRange::new_min(10)),
            EntitySelectorOption::Tag(false, "foo".to_string()),
            EntitySelectorOption::Tag(true, "bar".to_string()),
        ]);
        assert_eq!(
            selector.to_string(),
            "@a[gamemode=!creative, level=10.., tag=foo, tag=!bar]"
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(EntitySelector::default(), EntitySelector::s_no_options());
        assert_eq!(EntitySelector::default().to_string(), "@s");
    }
}
