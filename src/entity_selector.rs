use crate::command::enums::gamemode::Gamemode;
use crate::command::enums::sort::Sort;
use crate::range::{FloatRange, IntegerRange};
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum EntitySelectorVariable {
    /// Selects the nearest player from the command's execution. If there are multiple nearest players, caused by them being precisely the same distance away, the player who most recently joined the server is selected.
    P,
    /// Selects a random online player.
    R,
    /// Selects every online player (alive or dead).
    A,
    /// Selects all alive entities in loaded chunks, and all alive online players.
    E,
    /// Selects the entity (alive or not) that the command was executed as. It does not select anything if the command was not ran as an entity (e.g. from a command block or server console).
    S,
    /// Selects the nearest alive entity from the command's execution.
    N,
}

impl Display for EntitySelectorVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntitySelectorVariable::P => "p",
            EntitySelectorVariable::R => "r",
            EntitySelectorVariable::A => "a",
            EntitySelectorVariable::E => "e",
            EntitySelectorVariable::S => "s",
            EntitySelectorVariable::N => "n",
        }
        .fmt(f)
    }
}

fn fmt_b_tree_map<K: Display, V: Display>(
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum AdvancementChoiceType {
    Boolean(bool),
    Criterion(BTreeMap<String, bool>),
}

impl From<bool> for AdvancementChoiceType {
    fn from(value: bool) -> Self {
        AdvancementChoiceType::Boolean(value)
    }
}

impl From<BTreeMap<String, bool>> for AdvancementChoiceType {
    fn from(value: BTreeMap<String, bool>) -> Self {
        AdvancementChoiceType::Criterion(value)
    }
}

impl Display for AdvancementChoiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancementChoiceType::Boolean(bool) => bool.fmt(f),
            AdvancementChoiceType::Criterion(map) => fmt_b_tree_map(f, map),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
    pub fn can_be_repeated(&self) -> bool {
        matches!(
            self,
            EntitySelectorOption::Tag(..)
                | EntitySelectorOption::Nbt(..)
                | EntitySelectorOption::Predicate(..)
                | EntitySelectorOption::Name(true, _)
                | EntitySelectorOption::Team(true, _)
                | EntitySelectorOption::Type(true, _)
                | EntitySelectorOption::Gamemode(true, _)
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
            EntitySelectorOption::X(x) => write_entity_selector_option!(f, "x", x),
            EntitySelectorOption::Y(y) => write_entity_selector_option!(f, "y", y),
            EntitySelectorOption::Z(z) => write_entity_selector_option!(f, "z", z),
            EntitySelectorOption::Distance(d) => write_entity_selector_option!(f, "distance", d),
            EntitySelectorOption::DistanceX(dx) => write_entity_selector_option!(f, "dx", dx),
            EntitySelectorOption::DistanceY(dy) => write_entity_selector_option!(f, "dy", dy),
            EntitySelectorOption::DistanceZ(dz) => write_entity_selector_option!(f, "dz", dz),
            EntitySelectorOption::XRotation(rot) => {
                write_entity_selector_option!(f, "x_rotation", rot)
            }
            EntitySelectorOption::YRotation(rot) => {
                write_entity_selector_option!(f, "y_rotation", rot)
            }
            EntitySelectorOption::Level(level) => write_entity_selector_option!(f, "level", level),
            EntitySelectorOption::Limit(limit) => write_entity_selector_option!(f, "limit", limit),
            EntitySelectorOption::Sort(sort) => write_entity_selector_option!(f, "sort", sort),

            EntitySelectorOption::Tag(inv, val) => {
                write_entity_selector_option!(f, "tag", inv, val)
            }
            EntitySelectorOption::Team(inv, val) => {
                write_entity_selector_option!(f, "team", inv, val)
            }
            EntitySelectorOption::Name(inv, val) => {
                write_entity_selector_option!(f, "name", inv, val)
            }
            EntitySelectorOption::Type(inv, val) => {
                write_entity_selector_option!(f, "type", inv, val)
            }
            EntitySelectorOption::Predicate(inv, val) => {
                write_entity_selector_option!(f, "predicate", inv, val)
            }
            EntitySelectorOption::Nbt(inv, val) => {
                write_entity_selector_option!(f, "nbt", inv, val)
            }
            EntitySelectorOption::Gamemode(inv, val) => {
                write_entity_selector_option!(f, "gamemode", inv, val)
            }

            EntitySelectorOption::Scores(scores) => {
                f.write_str("scores=")?;
                fmt_b_tree_map(f, scores)
            }
            EntitySelectorOption::Advancements(advancements) => {
                f.write_str("advancements=")?;
                fmt_b_tree_map(f, advancements)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum EntitySelector {
    Variable(EntitySelectorVariable, Vec<EntitySelectorOption>),
    Name(String),
}

impl EntitySelector {
    #[inline]
    #[must_use]
    pub fn new(variable: EntitySelectorVariable, options: Vec<EntitySelectorOption>) -> Self {
        Self::Variable(variable, options)
    }

    #[inline]
    #[must_use]
    pub fn p(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::P, options)
    }

    #[inline]
    #[must_use]
    pub fn p_no_options() -> Self {
        Self::p(vec![])
    }

    #[inline]
    #[must_use]
    pub fn r(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::R, options)
    }

    #[inline]
    #[must_use]
    pub fn r_no_options() -> Self {
        Self::r(vec![])
    }

    #[inline]
    #[must_use]
    pub fn a(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::A, options)
    }

    #[inline]
    #[must_use]
    pub fn a_no_options() -> Self {
        Self::a(vec![])
    }

    #[inline]
    #[must_use]
    pub fn e(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::E, options)
    }

    #[inline]
    #[must_use]
    pub fn e_no_options() -> Self {
        Self::e(vec![])
    }

    #[inline]
    #[must_use]
    pub fn s(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::S, options)
    }

    #[inline]
    #[must_use]
    pub fn s_no_options() -> Self {
        Self::s(vec![])
    }

    #[inline]
    #[must_use]
    pub fn n(options: Vec<EntitySelectorOption>) -> Self {
        Self::new(EntitySelectorVariable::N, options)
    }

    #[inline]
    #[must_use]
    pub fn n_no_options() -> Self {
        Self::n(vec![])
    }
}

impl Default for EntitySelector {
    fn default() -> Self {
        EntitySelector::s_no_options()
    }
}

impl Display for EntitySelector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntitySelector::Variable(variable, options) => {
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
            EntitySelector::Name(name) => name.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
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
        compound.insert("OnGround".to_string(), SNBT::Byte(1));
        let nbt = SNBT::Compound(compound);
        assert_eq!(
            EntitySelector::e(vec![EntitySelectorOption::Nbt(false, nbt)]).to_string(),
            "@e[nbt={OnGround:1b}]"
        );
        let mut compound = BTreeMap::new();
        compound.insert("Air".to_string(), SNBT::Short(300));
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
