use crate::{
    resource_location::ResourceLocation,
    snbt::{SnbtCompound, fmt_snbt_compound},
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockState {
    pub id: ResourceLocation,
    pub block_states: BTreeMap<String, String>,
    pub data_tags: Option<SnbtCompound>,
}

impl Display for BlockState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.id.fmt(f)?;

        if !self.block_states.is_empty() {
            let states: Vec<String> = self
                .block_states
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            write!(f, "[{}]", states.join(", "))?;
        }

        if let Some(snbt) = &self.data_tags {
            fmt_snbt_compound(f, snbt)?;
        }

        Ok(())
    }
}

impl BlockState {
    #[must_use]
    pub const fn new(id: ResourceLocation) -> Self {
        Self {
            id,
            block_states: BTreeMap::new(),
            data_tags: None,
        }
    }
}
