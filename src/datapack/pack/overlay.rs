use crate::datapack::pack::format::Format;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Overlay {
    pub directory: String,
    pub formats: Option<Format>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Overlays {
    pub entries: Vec<Overlay>,
}
