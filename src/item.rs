use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    ResourceLocation(ResourceLocation),
    Wildcard,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::ResourceLocation(resource_location) => resource_location.fmt(f),
            ItemType::Wildcard => "*".fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NegatedTest(pub bool, pub ItemTest);

impl Display for NegatedTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            write!(f, "!")?;
        }
        self.1.fmt(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct OrGroup(pub Vec<NegatedTest>);

impl Display for OrGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self.0.iter().map(|t| t.to_string()).collect();
        write!(f, "{}", parts.join("|"))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemPredicate {
    id: ItemType,
    tests: Vec<OrGroup>,
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
        let negated_test = NegatedTest(negated, test);
        let group = OrGroup(vec![negated_test]);
        self.with_test_group(group)
    }
}
