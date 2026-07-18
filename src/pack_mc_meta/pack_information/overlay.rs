use serde::{Deserialize, Serialize};

use crate::pack_mc_meta::pack_information::format::Format;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OverlayEntry {
    pub directory: String,
    pub formats: Option<Format>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Overlays {
    pub entries: Vec<OverlayEntry>,
}
