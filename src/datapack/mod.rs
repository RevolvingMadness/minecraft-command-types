pub mod pack;
pub mod tag;

use crate::datapack::pack::Pack;
use crate::datapack::pack::feature::Features;
use crate::datapack::pack::filter::Filter;
use crate::datapack::pack::language::Language;
use crate::datapack::pack::overlay::Overlays;
use crate::datapack::tag::{Tag, Worldgen};
use crate::function::Function;
use crate::resource_location::{ResourceLocationPaths, ResourceLocationPathsRef};
use hashbrown::HashMap;
use hashbrown::hash_map::EntryRef;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PackMCMeta {
    pub pack: Pack,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Features>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlays: Option<Overlays>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<BTreeMap<String, Language>>,
}

#[derive(Debug, Clone)]
pub enum FilePathNode<T> {
    Directory(String, Vec<Self>),
    File(String, T),
}

impl<T> FilePathNode<T> {
    pub fn from_str(path: &str, value: T) -> Self {
        let mut parts = path.split('/').rev();
        let file_name = parts.next().expect("Path cannot be empty");
        let mut current_node = Self::File(file_name.to_string(), value);

        for part in parts {
            current_node = Self::Directory(part.to_string(), vec![current_node]);
        }

        current_node
    }

    pub fn from_vec_string(vec: Vec<String>, value: T) -> Self {
        let mut vec = vec.into_iter().rev();

        let file_name = vec.next().expect("Path cannot be empty");
        let mut current_node = Self::File(file_name, value);

        for part in vec {
            current_node = Self::Directory(part, vec![current_node]);
        }

        current_node
    }
}

pub type TagRegistry = String;

#[derive(Debug, Clone, Default)]
pub struct Namespace {
    pub functions: HashMap<ResourceLocationPaths, Function>,
    pub tags: HashMap<TagRegistry, HashMap<ResourceLocationPaths, Tag>>,

    pub advancements: HashMap<ResourceLocationPaths, Value>,
    pub banner_patterns: HashMap<ResourceLocationPaths, Value>,
    pub cat_variants: HashMap<ResourceLocationPaths, Value>,
    pub chat_types: HashMap<ResourceLocationPaths, Value>,
    pub chicken_variants: HashMap<ResourceLocationPaths, Value>,
    pub cow_variants: HashMap<ResourceLocationPaths, Value>,
    pub damage_types: HashMap<ResourceLocationPaths, Value>,
    pub dialogs: HashMap<ResourceLocationPaths, Value>,
    pub dimensions: HashMap<ResourceLocationPaths, Value>,
    pub dimension_types: HashMap<ResourceLocationPaths, Value>,
    pub enchantments: HashMap<ResourceLocationPaths, Value>,
    pub enchantment_providers: HashMap<ResourceLocationPaths, Value>,
    pub frog_variants: HashMap<ResourceLocationPaths, Value>,
    pub instruments: HashMap<ResourceLocationPaths, Value>,
    pub item_modifiers: HashMap<ResourceLocationPaths, Value>,
    pub jukebox_songs: HashMap<ResourceLocationPaths, Value>,
    pub loot_tables: HashMap<ResourceLocationPaths, Value>,
    pub painting_variants: HashMap<ResourceLocationPaths, Value>,
    pub pig_variants: HashMap<ResourceLocationPaths, Value>,
    pub predicates: HashMap<ResourceLocationPaths, Value>,
    pub recipes: HashMap<ResourceLocationPaths, Value>,
    pub test_environments: HashMap<ResourceLocationPaths, Value>,
    pub test_instances: HashMap<ResourceLocationPaths, Value>,
    pub timelines: HashMap<ResourceLocationPaths, Value>,
    pub trial_spawners: HashMap<ResourceLocationPaths, Value>,
    pub trim_materials: HashMap<ResourceLocationPaths, Value>,
    pub trim_patterns: HashMap<ResourceLocationPaths, Value>,
    pub wolf_sound_variants: HashMap<ResourceLocationPaths, Value>,
    pub wolf_variants: HashMap<ResourceLocationPaths, Value>,
    pub worldgen: Worldgen,
}

fn write_file_path_nodes<T>(
    base_path: &Path,
    nodes: &HashMap<ResourceLocationPaths, T>,
    extension: &str,
    serializer: &impl Fn(&T) -> io::Result<String>,
) -> io::Result<()> {
    for (path, content) in nodes {
        let mut file_path = PathBuf::from(base_path);

        for segment in path {
            file_path.push(segment);
        }

        file_path.set_extension(extension.trim_start_matches('.'));

        let serialized_content = serializer(content)?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, serialized_content)?;
    }

    Ok(())
}

impl Namespace {
    pub fn merge(&mut self, other: Self) {
        self.functions.extend(other.functions);

        for (tag_type, tags) in other.tags {
            self.tags.entry(tag_type).or_default().extend(tags);
        }

        self.advancements.extend(other.advancements);
        self.banner_patterns.extend(other.banner_patterns);
        self.cat_variants.extend(other.cat_variants);
        self.chat_types.extend(other.chat_types);
        self.chicken_variants.extend(other.chicken_variants);
        self.cow_variants.extend(other.cow_variants);
        self.damage_types.extend(other.damage_types);
        self.dialogs.extend(other.dialogs);
        self.dimensions.extend(other.dimensions);
        self.dimension_types.extend(other.dimension_types);
        self.enchantments.extend(other.enchantments);
        self.enchantment_providers
            .extend(other.enchantment_providers);
        self.frog_variants.extend(other.frog_variants);
        self.instruments.extend(other.instruments);
        self.item_modifiers.extend(other.item_modifiers);
        self.jukebox_songs.extend(other.jukebox_songs);
        self.loot_tables.extend(other.loot_tables);
        self.painting_variants.extend(other.painting_variants);
        self.pig_variants.extend(other.pig_variants);
        self.predicates.extend(other.predicates);
        self.recipes.extend(other.recipes);
        self.test_environments.extend(other.test_environments);
        self.test_instances.extend(other.test_instances);
        self.timelines.extend(other.timelines);
        self.trial_spawners.extend(other.trial_spawners);
        self.trim_materials.extend(other.trim_materials);
        self.trim_patterns.extend(other.trim_patterns);
        self.wolf_sound_variants.extend(other.wolf_sound_variants);
        self.wolf_variants.extend(other.wolf_variants);

        self.worldgen.merge(other.worldgen);
    }

    pub fn write(&self, namespace_path: &Path) -> io::Result<()> {
        let json_serializer = |v: &Value| serde_json::to_string_pretty(v).map_err(io::Error::other);

        write_file_path_nodes(
            &namespace_path.join("function"),
            &self.functions,
            ".mcfunction",
            &|function| Ok(function.to_string()),
        )?;

        let tags_root_path = namespace_path.join("tags");

        for (registry, nodes) in &self.tags {
            let type_path = tags_root_path.join(registry);

            write_file_path_nodes(&type_path, nodes, ".json", &|tag| {
                serde_json::to_string_pretty(tag).map_err(io::Error::other)
            })?;
        }

        macro_rules! generate_write_file_path_nodes {
            ($field_name:expr, $folder_name:expr) => {
                write_file_path_nodes(
                    &namespace_path.join($folder_name),
                    &$field_name,
                    ".json",
                    &json_serializer,
                )?;
            };
        }

        generate_write_file_path_nodes!(self.advancements, "advancement");
        generate_write_file_path_nodes!(self.banner_patterns, "banner_pattern");
        generate_write_file_path_nodes!(self.cat_variants, "cat_variant");
        generate_write_file_path_nodes!(self.chat_types, "chat_type");
        generate_write_file_path_nodes!(self.chicken_variants, "chicken_variant");
        generate_write_file_path_nodes!(self.cow_variants, "cow_variant");
        generate_write_file_path_nodes!(self.damage_types, "damage_type");
        generate_write_file_path_nodes!(self.dialogs, "dialog");
        generate_write_file_path_nodes!(self.dimensions, "dimension");
        generate_write_file_path_nodes!(self.dimension_types, "dimension_type");
        generate_write_file_path_nodes!(self.enchantments, "enchantment");
        generate_write_file_path_nodes!(self.enchantment_providers, "enchantment_provider");
        generate_write_file_path_nodes!(self.frog_variants, "frog_variant");
        generate_write_file_path_nodes!(self.instruments, "instrument");
        generate_write_file_path_nodes!(self.item_modifiers, "item_modifier");
        generate_write_file_path_nodes!(self.jukebox_songs, "jukebox_song");
        generate_write_file_path_nodes!(self.loot_tables, "loot_table");
        generate_write_file_path_nodes!(self.painting_variants, "painting_variant");
        generate_write_file_path_nodes!(self.pig_variants, "pig_variant");
        generate_write_file_path_nodes!(self.predicates, "predicate");
        generate_write_file_path_nodes!(self.recipes, "recipe");
        generate_write_file_path_nodes!(self.test_environments, "test_environment");
        generate_write_file_path_nodes!(self.test_instances, "test_instance");
        generate_write_file_path_nodes!(self.timelines, "timeline");
        generate_write_file_path_nodes!(self.trial_spawners, "trial_spawner");
        generate_write_file_path_nodes!(self.trim_materials, "trim_material");
        generate_write_file_path_nodes!(self.trim_patterns, "trim_pattern");
        generate_write_file_path_nodes!(self.wolf_sound_variants, "wolf_sound_variant");
        generate_write_file_path_nodes!(self.wolf_variants, "wolf_variant");

        Ok(())
    }

    pub fn add_tag(&mut self, registry: TagRegistry, path: &ResourceLocationPaths, tag: Tag) {
        if let Some(original_tags) = self.tags.get_mut(&registry) {
            if let Some(original_tag) = original_tags.get_mut(path) {
                original_tag.extend(tag);
            } else {
                original_tags.insert(path.clone(), tag);
            }
        } else {
            self.tags
                .insert(registry, HashMap::from([(path.clone(), tag)]));
        }
    }

    #[must_use]
    pub fn get_function(&mut self, paths: ResourceLocationPathsRef) -> &Function {
        self.functions.entry_ref(paths).or_default()
    }

    #[must_use]
    pub fn get_function_mut(&mut self, paths: ResourceLocationPathsRef) -> &mut Function {
        self.functions.entry_ref(paths).or_default()
    }
}

#[derive(Debug, Clone)]
pub struct Datapack {
    pub pack: PackMCMeta,
    pub namespaces: HashMap<String, Namespace>,
}

impl Datapack {
    #[must_use]
    pub fn new(pack_format: i32, description: Value) -> Self {
        Self {
            pack: PackMCMeta {
                pack: Pack {
                    pack_format: Some(pack_format),
                    description,
                    max_format: None,
                    min_format: None,
                    supported_formats: None,
                },
                features: None,
                filter: None,
                overlays: None,
                language: None,
            },
            namespaces: HashMap::new(),
        }
    }

    #[must_use]
    pub fn new_pack(pack: PackMCMeta) -> Self {
        Self {
            pack,
            namespaces: HashMap::new(),
        }
    }

    pub fn write<P: AsRef<Path>>(&self, output: P) -> io::Result<()> {
        let output = output.as_ref();

        fs::create_dir_all(output)?;

        let mcmeta_path = output.join("pack.mcmeta");
        let mcmeta_content = serde_json::to_string_pretty(&self.pack).map_err(io::Error::other)?;
        fs::write(mcmeta_path, mcmeta_content)?;

        let data_path = output.join("data");

        for (name, namespace) in &self.namespaces {
            let namespace_path = data_path.join(name);

            namespace.write(&namespace_path)?;
        }

        Ok(())
    }

    pub fn get_namespace_mut(&mut self, name: &str) -> &mut Namespace {
        self.namespaces.entry_ref(name).or_default()
    }

    pub fn add_namespace(&mut self, name: &str, namespace: Namespace) {
        match self.namespaces.entry_ref(name) {
            EntryRef::Occupied(mut entry) => {
                entry.get_mut().merge(namespace);
            }
            EntryRef::Vacant(entry) => {
                entry.insert(namespace);
            }
        }
    }
}
