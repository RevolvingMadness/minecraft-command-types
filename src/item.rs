use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ItemTest {
    Component(ResourceLocation),
    ComponentMatches(ResourceLocation, SNBT),
    Predicate(ResourceLocation, SNBT),
}

impl Display for ItemTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemTest::Component(id) => id.fmt(f),
            ItemTest::ComponentMatches(id, value) => write!(f, "{}={}", id, value),
            ItemTest::Predicate(id, value) => write!(f, "{}~{}", id, value),
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
            ItemType::ResourceLocation(resource_location) => resource_location.fmt(f),
            ItemType::Wildcard => f.write_str("*"),
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
    pub tests: Vec<OrGroup>,
}

impl Display for ItemPredicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)?;

        if self.tests.is_empty() {
            return Ok(());
        }

        write!(f, "[")?;

        let parts: Vec<String> = self.tests.iter().map(|g| g.to_string()).collect();
        write!(f, "{}]", parts.join(","))
    }
}

impl ItemPredicate {
    pub fn new(id: ItemType) -> Self {
        Self {
            id,
            tests: Vec::new(),
        }
    }

    pub fn with_test_group(mut self, group: OrGroup) -> Self {
        self.tests.push(group);
        self
    }

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
            ItemComponent::KeyValue(component, value) => {
                write!(f, "{}={}", component, value)
            }
            ItemComponent::Remove(component) => write!(f, "!{}", component),
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
