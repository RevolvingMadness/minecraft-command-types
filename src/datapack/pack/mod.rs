pub mod feature;
pub mod filter;
pub mod format;
pub mod language;
pub mod overlay;

use crate::datapack::pack::format::Format;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pack {
    pub description: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_format: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<Format>,
}
