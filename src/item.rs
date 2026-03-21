use crate::snbt::SNBT;
use crate::{macroable::Macroable, resource_location::ResourceLocation};
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ItemTest {
    Component(ResourceLocation),
    ComponentMatches(ResourceLocation, Macroable<SNBT>),
    Predicate(ResourceLocation, Macroable<SNBT>),
}

impl Display for ItemTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Component(id) => id.fmt(f),
            Self::ComponentMatches(id, value) => write!(f, "{}={}", id, value),
            Self::Predicate(id, value) => write!(f, "{}~{}", id, value),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ItemType {
    ResourceLocation(ResourceLocation),
    Wildcard,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResourceLocation(resource_location) => resource_location.fmt(f),
            Self::Wildcard => f.write_str("*"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct OrGroup(pub Vec<(bool, ItemTest)>);

impl Display for OrGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self
            .0
            .iter()
            .map(|(negated, test)| {
                if *negated {
                    format!("!{}", test)
                } else {
                    test.to_string()
                }
            })
            .collect();
        write!(f, "{}", parts.join("|"))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct ItemPredicate {
    pub id: ItemType,
    pub or_groups: Vec<OrGroup>,
}

impl Display for ItemPredicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)?;

        if self.or_groups.is_empty() {
            return Ok(());
        }

        write!(f, "[")?;

        let parts: Vec<String> = self.or_groups.iter().map(ToString::to_string).collect();
        write!(f, "{}]", parts.join(","))
    }
}

impl ItemPredicate {
    #[must_use]
    pub const fn new(id: ItemType) -> Self {
        Self {
            id,
            or_groups: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_test_group(mut self, group: OrGroup) -> Self {
        self.or_groups.push(group);

        self
    }

    #[must_use]
    pub fn with_test(self, negated: bool, test: ItemTest) -> Self {
        let group = OrGroup(vec![(negated, test)]);

        self.with_test_group(group)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ItemComponent {
    KeyValue(ResourceLocation, SNBT),
    Remove(ResourceLocation),
}

impl Display for ItemComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyValue(component, value) => {
                write!(f, "{}={}", component, value)
            }
            Self::Remove(component) => write!(f, "!{}", component),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct ItemStack {
    pub id: ItemType,
    pub components: Vec<ItemComponent>,
}

impl Display for ItemStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)?;

        if !self.components.is_empty() {
            write!(f, "[")?;
            let mut first = true;

            for component in &self.components {
                if !first {
                    write!(f, ", ")?;
                }

                write!(f, "{}", component)?;

                first = false;
            }

            write!(f, "]")?;
        }

        Ok(())
    }
}
