pub mod feature;
pub mod filter;
pub mod format;
pub mod language;
pub mod overlay;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pack_mc_meta::pack_information::format::Format;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInformation {
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
