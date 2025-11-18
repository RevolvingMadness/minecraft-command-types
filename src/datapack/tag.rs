use crate::datapack::FilePathNode;
use crate::resource_location::ResourceLocation;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Display, EnumString, Serialize, Deserialize, Ord, PartialOrd,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TagType {
    BannerPattern,
    Block,
    DamageType,
    Dialog,
    Enchantment,
    EntityType,
    Fluid,
    GameEvent,
    Instrument,
    Item,
    PaintingVariant,
    PointOfInterestType,
    Timeline,
    Biome,
    FlatLevelGeneratorPreset,
    Structure,
    WorldPreset,
}

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

impl TagType {
    pub fn is_worldgen(&self) -> bool {
        matches!(
            self,
            TagType::Biome
                | TagType::FlatLevelGeneratorPreset
                | TagType::Structure
                | TagType::WorldPreset
        )
    }
}

#[derive(Clone, Default)]
pub struct Worldgen {
    pub biome: Vec<FilePathNode<Value>>,
    pub configured_carver: Vec<FilePathNode<Value>>,
    pub configured_feature: Vec<FilePathNode<Value>>,
    pub density_function: Vec<FilePathNode<Value>>,
    pub noise: Vec<FilePathNode<Value>>,
    pub noise_settings: Vec<FilePathNode<Value>>,
    pub placed_feature: Vec<FilePathNode<Value>>,
    pub processor_list: Vec<FilePathNode<Value>>,
    pub structure: Vec<FilePathNode<Value>>,
    pub structure_set: Vec<FilePathNode<Value>>,
    pub template_pool: Vec<FilePathNode<Value>>,
    pub world_preset: Vec<FilePathNode<Value>>,
    pub flat_level_generator_preset: Vec<FilePathNode<Value>>,
    pub multi_noise_biome_source_parameter_list: Vec<FilePathNode<Value>>,
}
