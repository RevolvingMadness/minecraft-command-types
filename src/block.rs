use crate::has_macro::HasMacro;
use crate::nbt_path::SNBTCompound;
use crate::resource_location::ResourceLocation;
use crate::snbt::fmt_snbt_compound;
use minecraft_command_types_proc_macros::HasMacro;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct BlockState {
    pub id: ResourceLocation,
    pub block_states: BTreeMap<String, String>,
    pub data_tags: Option<SNBTCompound>,
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
            fmt_snbt_compound(f, snbt)?;
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
