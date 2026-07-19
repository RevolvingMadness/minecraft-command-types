use serde_json::Value;

use crate::datapack::FilePathNode;

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
