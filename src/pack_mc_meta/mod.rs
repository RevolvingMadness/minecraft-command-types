use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::pack_mc_meta::pack_information::{
    PackInformation, feature::Features, filter::Filter, language::Language, overlay::Overlays,
};

pub mod pack_information;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackMcMeta {
    #[serde(rename = "pack")]
    pub information: PackInformation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Features>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays: Option<Overlays>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<BTreeMap<String, Language>>,
}
