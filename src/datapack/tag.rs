use crate::resource_location::ResourceLocation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagValue {
    ResourceLocation(ResourceLocation),
    Id(String),
    Explicit {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    pub values: Vec<TagValue>,
}

impl Tag {
    pub fn extend(&mut self, other: Self) {
        if self.replace.is_none_or(|replace| !replace) {
            self.replace = other.replace;
        }

        self.values.extend(other.values);
    }
}
