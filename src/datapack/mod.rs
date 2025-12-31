pub mod pack;
pub mod tag;

use crate::datapack::pack::Pack;
use crate::datapack::pack::feature::Features;
use crate::datapack::pack::filter::Filter;
use crate::datapack::pack::language::Language;
use crate::datapack::pack::overlay::Overlays;
use crate::datapack::tag::{Tag, TagType, Worldgen};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
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

#[derive(Debug, Clone, Default)]
pub struct Namespace {
    pub functions: BTreeMap<NonEmpty<String>, String>,
    pub tags: BTreeMap<TagType, BTreeMap<NonEmpty<String>, Tag>>,

    pub advancements: BTreeMap<NonEmpty<String>, Value>,
    pub banner_patterns: BTreeMap<NonEmpty<String>, Value>,
    pub cat_variants: BTreeMap<NonEmpty<String>, Value>,
    pub chat_types: BTreeMap<NonEmpty<String>, Value>,
    pub chicken_variants: BTreeMap<NonEmpty<String>, Value>,
    pub cow_variants: BTreeMap<NonEmpty<String>, Value>,
    pub damage_types: BTreeMap<NonEmpty<String>, Value>,
    pub dialogs: BTreeMap<NonEmpty<String>, Value>,
    pub dimensions: BTreeMap<NonEmpty<String>, Value>,
    pub dimension_types: BTreeMap<NonEmpty<String>, Value>,
    pub enchantments: BTreeMap<NonEmpty<String>, Value>,
    pub enchantment_providers: BTreeMap<NonEmpty<String>, Value>,
    pub frog_variants: BTreeMap<NonEmpty<String>, Value>,
    pub instruments: BTreeMap<NonEmpty<String>, Value>,
    pub item_modifiers: BTreeMap<NonEmpty<String>, Value>,
    pub jukebox_songs: BTreeMap<NonEmpty<String>, Value>,
    pub loot_tables: BTreeMap<NonEmpty<String>, Value>,
    pub painting_variants: BTreeMap<NonEmpty<String>, Value>,
    pub pig_variants: BTreeMap<NonEmpty<String>, Value>,
    pub predicates: BTreeMap<NonEmpty<String>, Value>,
    pub recipes: BTreeMap<NonEmpty<String>, Value>,
    pub test_environments: BTreeMap<NonEmpty<String>, Value>,
    pub test_instances: BTreeMap<NonEmpty<String>, Value>,
    pub timelines: BTreeMap<NonEmpty<String>, Value>,
    pub trial_spawners: BTreeMap<NonEmpty<String>, Value>,
    pub trim_materials: BTreeMap<NonEmpty<String>, Value>,
    pub trim_patterns: BTreeMap<NonEmpty<String>, Value>,
    pub wolf_sound_variants: BTreeMap<NonEmpty<String>, Value>,
    pub wolf_variants: BTreeMap<NonEmpty<String>, Value>,
    pub worldgen: Worldgen,
}
fn write_file_path_nodes<T>(
    base_path: &Path,
    nodes: &BTreeMap<NonEmpty<String>, T>,
    extension: &str,
    serializer: &impl Fn(&T) -> io::Result<String>,
) -> io::Result<()> {
    for (path, content) in nodes {
        let mut file_path = PathBuf::from(base_path);

        for segment in path.iter() {
            file_path.push(segment);
        }

        file_path.set_extension(extension.trim_start_matches('.'));

        let serialized_content = serializer(content)?;
        if serialized_content.is_empty() {
            continue;
        }

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, serialized_content)?;
    }

    Ok(())
}

impl Namespace {
    pub fn merge(&mut self, mut other: Namespace) {
        self.functions.append(&mut other.functions);

        for (tag_type, tags) in other.tags {
            self.tags.entry(tag_type).or_default().extend(tags);
        }

        self.advancements.append(&mut other.advancements);
        self.banner_patterns.append(&mut other.banner_patterns);
        self.cat_variants.append(&mut other.cat_variants);
        self.chat_types.append(&mut other.chat_types);
        self.chicken_variants.append(&mut other.chicken_variants);
        self.cow_variants.append(&mut other.cow_variants);
        self.damage_types.append(&mut other.damage_types);
        self.dialogs.append(&mut other.dialogs);
        self.dimensions.append(&mut other.dimensions);
        self.dimension_types.append(&mut other.dimension_types);
        self.enchantments.append(&mut other.enchantments);
        self.enchantment_providers
            .append(&mut other.enchantment_providers);
        self.frog_variants.append(&mut other.frog_variants);
        self.instruments.append(&mut other.instruments);
        self.item_modifiers.append(&mut other.item_modifiers);
        self.jukebox_songs.append(&mut other.jukebox_songs);
        self.loot_tables.append(&mut other.loot_tables);
        self.painting_variants.append(&mut other.painting_variants);
        self.pig_variants.append(&mut other.pig_variants);
        self.predicates.append(&mut other.predicates);
        self.recipes.append(&mut other.recipes);
        self.test_environments.append(&mut other.test_environments);
        self.test_instances.append(&mut other.test_instances);
        self.timelines.append(&mut other.timelines);
        self.trial_spawners.append(&mut other.trial_spawners);
        self.trim_materials.append(&mut other.trim_materials);
        self.trim_patterns.append(&mut other.trim_patterns);
        self.wolf_sound_variants
            .append(&mut other.wolf_sound_variants);
        self.wolf_variants.append(&mut other.wolf_variants);

        self.worldgen.merge(other.worldgen);
    }

    pub fn write(&self, namespace_path: &Path) -> io::Result<()> {
        let json_serializer = |v: &Value| serde_json::to_string_pretty(v).map_err(io::Error::other);

        write_file_path_nodes(
            &namespace_path.join("function"),
            &self.functions,
            ".mcfunction",
            &|content_str| Ok(content_str.clone()),
        )?;

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

    pub fn add_tag(&mut self, tag_type: TagType, path: &NonEmpty<String>, new_tag: Tag) {
        if let Some(tags) = self.tags.get_mut(&tag_type) {
            if let Some(tag) = tags.get_mut(path) {
                tag.extend(new_tag);
            } else {
                tags.insert(path.clone(), new_tag);
            }
        } else {
            self.tags
                .insert(tag_type, BTreeMap::from([(path.clone(), new_tag)]));
        }
    }

    pub fn add_function(&mut self, path: &NonEmpty<String>, new_function: &str) {
        if let Some(functions) = self.functions.get_mut(path) {
            functions.push('\n');
            functions.push_str(new_function);
        } else {
            self.functions
                .insert(path.clone(), new_function.to_string());
        }
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

    pub fn add_namespace<T: ToString>(&mut self, name: T, namespace: Namespace) {
        match self.namespaces.entry(name.to_string()) {
            Entry::Vacant(e) => {
                e.insert(namespace);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().merge(namespace);
            }
        }
    }
}
