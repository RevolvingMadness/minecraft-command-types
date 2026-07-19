use serde::{Deserialize, Serialize};
use std::{collections::HashMap, slice};

use crate::resource_location::ResourceLocation;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockstateVariant {
    pub model: ResourceLocation,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub z: Option<i32>,
    pub uvlock: Option<bool>,
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockstateVariants {
    Single(BlockstateVariant),
    Multiple(Vec<BlockstateVariant>),
}

impl BlockstateVariants {
    #[must_use]
    pub const fn variants(&self) -> &[BlockstateVariant] {
        match self {
            Self::Single(variant) => slice::from_ref(variant),
            Self::Multiple(variants) => variants.as_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MultipartCondition {
    Group(HashMap<String, String>),
    Or {
        #[serde(rename = "OR")]
        or: Vec<Self>,
    },
    And {
        #[serde(rename = "AND")]
        and: Vec<Self>,
    },
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlockstateMultipart {
    pub apply: BlockstateVariants,
    pub when: Option<MultipartCondition>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockstateDefinition {
    pub variants: Option<HashMap<String, BlockstateVariants>>,
    pub multipart: Option<Vec<BlockstateMultipart>>,
}
