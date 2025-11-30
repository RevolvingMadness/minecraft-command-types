pub mod pack;
pub mod tag;

use crate::datapack::pack::feature::Features;
use crate::datapack::pack::filter::Filter;
use crate::datapack::pack::language::Language;
use crate::datapack::pack::overlay::Overlays;
use crate::datapack::pack::Pack;
use crate::datapack::tag::{Tag, TagType, Worldgen};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;
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

#[derive(Clone)]
pub enum FilePathNode<T> {
    Directory(String, Vec<FilePathNode<T>>),
    File(String, T),
}

impl<T> FilePathNode<T> {
    pub fn from_str(path: &str, value: T) -> Self {
        let mut parts = path.split('/').rev();
        let file_name = parts.next().expect("Path cannot be empty");
        let mut current_node = FilePathNode::File(file_name.to_string(), value);

        for part in parts {
            current_node = FilePathNode::Directory(part.to_string(), vec![current_node]);
        }

        current_node
    }

    pub fn from_nonempty_vec_string(vec: &NonEmpty<String>, value: T) -> Self {
        let mut vec = vec.iter().rev();

        let file_name = vec.next().expect("Path cannot be empty");
        let mut current_node = FilePathNode::File(file_name.to_string(), value);

        for part in vec {
            current_node = FilePathNode::Directory(part.to_string(), vec![current_node]);
        }

        current_node
    }
}

#[derive(Clone, Default)]
pub struct Namespace {
    pub functions: Vec<FilePathNode<String>>,
    pub tags: BTreeMap<TagType, Vec<FilePathNode<Tag>>>,

    pub advancements: Vec<FilePathNode<Value>>,
    pub banner_patterns: Vec<FilePathNode<Value>>,
    pub cat_variants: Vec<FilePathNode<Value>>,
    pub chat_types: Vec<FilePathNode<Value>>,
    pub chicken_variants: Vec<FilePathNode<Value>>,
    pub cow_variants: Vec<FilePathNode<Value>>,
    pub damage_types: Vec<FilePathNode<Value>>,
    pub dialogs: Vec<FilePathNode<Value>>,
    pub dimensions: Vec<FilePathNode<Value>>,
    pub dimension_types: Vec<FilePathNode<Value>>,
    pub enchantments: Vec<FilePathNode<Value>>,
    pub enchantment_providers: Vec<FilePathNode<Value>>,
    pub frog_variants: Vec<FilePathNode<Value>>,
    pub instruments: Vec<FilePathNode<Value>>,
    pub item_modifiers: Vec<FilePathNode<Value>>,
    pub jukebox_songs: Vec<FilePathNode<Value>>,
    pub loot_tables: Vec<FilePathNode<Value>>,
    pub painting_variants: Vec<FilePathNode<Value>>,
    pub pig_variants: Vec<FilePathNode<Value>>,
    pub predicates: Vec<FilePathNode<Value>>,
    pub recipes: Vec<FilePathNode<Value>>,
    pub test_environments: Vec<FilePathNode<Value>>,
    pub test_instances: Vec<FilePathNode<Value>>,
    pub timelines: Vec<FilePathNode<Value>>,
    pub trial_spawners: Vec<FilePathNode<Value>>,
    pub trim_materials: Vec<FilePathNode<Value>>,
    pub trim_patterns: Vec<FilePathNode<Value>>,
    pub wolf_sound_variants: Vec<FilePathNode<Value>>,
    pub wolf_variants: Vec<FilePathNode<Value>>,
    pub worldgen: Worldgen,
}

fn write_file_path_nodes<T>(
    base_path: &Path,
    nodes: &[FilePathNode<T>],
    extension: &str,
    serializer: &impl Fn(&T) -> io::Result<String>,
) -> io::Result<()> {
    fs::create_dir_all(base_path)?;
    for node in nodes {
        match node {
            FilePathNode::Directory(name, children) => {
                let dir_path = base_path.join(name);
                write_file_path_nodes(&dir_path, children, extension, serializer)?;
            }
            FilePathNode::File(name, content) => {
                let filename = if name.ends_with(extension) {
                    name.clone()
                } else {
                    format!("{}{}", name, extension)
                };
                let file_path = base_path.join(filename);
                let serialized_content = serializer(content)?;
                fs::write(file_path, serialized_content)?;
            }
        }
    }
    Ok(())
}

impl Namespace {
    pub fn write(&self, namespace_path: &Path) -> io::Result<()> {
        let json_serializer = |v: &Value| serde_json::to_string_pretty(v).map_err(io::Error::other);

        if !self.functions.is_empty() {
            write_file_path_nodes(
                &namespace_path.join("function"),
                &self.functions,
                ".mcfunction",
                &|content_str| Ok(content_str.clone()),
            )?;
        }

        if !self.tags.is_empty() {
            let tags_root_path = namespace_path.join("tags");
            for (tag_type, nodes) in &self.tags {
                let type_path = if tag_type.is_worldgen() {
                    tags_root_path.join("worldgen").join(tag_type.to_string())
                } else {
                    tags_root_path.join(tag_type.to_string())
                };

                write_file_path_nodes(&type_path, nodes, ".json", &|tag| {
                    serde_json::to_string_pretty(tag).map_err(io::Error::other)
                })?;
            }
        }

        macro_rules! generate_write_file_path_nodes {
            ($field_name:expr, $folder_name:expr) => {
                if !$field_name.is_empty() {
                    write_file_path_nodes(
                        &namespace_path.join($folder_name),
                        &$field_name,
                        ".json",
                        &json_serializer,
                    )?;
                }
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
}

pub struct Datapack {
    pub pack: PackMCMeta,
    pub namespaces: BTreeMap<String, Namespace>,
}

impl Datapack {
    #[inline]
    #[must_use]
    pub fn new(pack_format: i32, description: Value) -> Datapack {
        Datapack {
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
            namespaces: BTreeMap::new(),
        }
    }
    pub fn write(&self, datapack_directory: &Path) -> io::Result<()> {
        fs::create_dir_all(datapack_directory)?;

        let mcmeta_path = datapack_directory.join("pack.mcmeta");
        let mcmeta_content = serde_json::to_string_pretty(&self.pack).map_err(io::Error::other)?;
        fs::write(mcmeta_path, mcmeta_content)?;

        let data_path = datapack_directory.join("data");

        for (name, namespace) in &self.namespaces {
            let namespace_path = data_path.join(name);
            namespace.write(&namespace_path)?;
        }

        Ok(())
    }

    pub fn get_namespace_mut(&mut self, name: &str) -> &mut Namespace {
        self.namespaces.entry(name.to_string()).or_default()
    }

    pub fn add_namespace(&mut self, name: String, namespace: Namespace) {
        self.namespaces.insert(name, namespace);
    }
}
