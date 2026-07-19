use itertools::Itertools;

use crate::{resource_location::ResourceLocation, snbt::Snbt};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemTest {
    Component(ResourceLocation),
    ComponentMatches(ResourceLocation, Snbt),
    Predicate(ResourceLocation, Snbt),
}

impl Display for ItemTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component(id) => id.fmt(f),
            Self::ComponentMatches(id, value) => write!(f, "{}={}", id, value),
            Self::Predicate(id, value) => write!(f, "{}~{}", id, value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemType {
    ResourceLocation(ResourceLocation),
    Wildcard,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceLocation(resource_location) => resource_location.fmt(f),
            Self::Wildcard => f.write_str("*"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrGroup(pub Vec<(bool, ItemTest)>);

impl Display for OrGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, (inverted, test)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, "|")?;
            }

            if *inverted {
                write!(f, "!")?;
            }

            write!(f, "{}", test)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemPredicate {
    pub id: ItemType,
    pub or_groups: Vec<OrGroup>,
}

impl Display for ItemPredicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)?;

        if !self.or_groups.is_empty() {
            write!(f, "[{}]", self.or_groups.iter().format("|"))?;
        }

        Ok(())
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemComponent {
    KeyValue(ResourceLocation, Snbt),
    Remove(ResourceLocation),
}

impl Display for ItemComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyValue(component, value) => {
                write!(f, "{}={}", component, value)
            }
            Self::Remove(component) => write!(f, "!{}", component),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemStack {
    pub id: ItemType,
    pub components: Vec<ItemComponent>,
}

impl Display for ItemStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)?;

        if !self.components.is_empty() {
            write!(f, "[{}]", self.components.iter().format(", "))?;
        }

        Ok(())
    }
}
