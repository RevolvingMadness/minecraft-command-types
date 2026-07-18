use crate::datapack::FilePathNode;
use crate::resource_location::ResourceLocation;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Clone, Default)]
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

impl Worldgen {
    pub fn merge(&mut self, other: Self) {
        self.biome.extend(other.biome);
        self.configured_carver.extend(other.configured_carver);
        self.configured_feature.extend(other.configured_feature);
        self.density_function.extend(other.density_function);
        self.noise.extend(other.noise);
        self.noise_settings.extend(other.noise_settings);
        self.placed_feature.extend(other.placed_feature);
        self.processor_list.extend(other.processor_list);
        self.structure.extend(other.structure);
        self.structure_set.extend(other.structure_set);
        self.template_pool.extend(other.template_pool);
        self.world_preset.extend(other.world_preset);
        self.flat_level_generator_preset
            .extend(other.flat_level_generator_preset);
        self.multi_noise_biome_source_parameter_list
            .extend(other.multi_noise_biome_source_parameter_list);
    }
}
