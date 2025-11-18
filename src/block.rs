use crate::has_macro::HasMacro;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_proc_macros::HasMacro;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct BlockPredicate {
    id: ResourceLocation,
    block_states: BTreeMap<String, String>,
    data_tags: Option<SNBT>,
}

impl Display for BlockPredicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            snbt.fmt(f)?;
        }

        Ok(())
    }
}

impl BlockPredicate {
    pub fn new(id: ResourceLocation) -> Self {
        Self {
            id,
            block_states: BTreeMap::new(),
            data_tags: None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct BlockState {
    id: ResourceLocation,
    block_states: BTreeMap<String, String>,
    data_tags: Option<SNBT>,
}

impl Display for BlockState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            snbt.fmt(f)?;
        }

        Ok(())
    }
}

impl BlockState {
    pub fn new(id: ResourceLocation) -> Self {
        Self {
            id,
            block_states: BTreeMap::new(),
            data_tags: None,
        }
    }
}
