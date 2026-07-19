use crate::{
    datapack::namespace::DatapackNamespace,
    pack_mc_meta::{PackMcMeta, pack_information::PackInformation},
    resource_location::ResourceLocationPaths,
};
use hashbrown::{HashMap, hash_map::EntryRef};
use serde_json::Value;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub mod function;
pub mod namespace;
pub mod tag;
pub mod worldgen;

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

#[derive(Debug, Clone)]
pub struct Datapack {
    pub pack_mc_meta: PackMcMeta,
    pub namespaces: HashMap<String, DatapackNamespace>,
}

impl Datapack {
    #[must_use]
    pub fn new(pack_format: i32, description: String) -> Self {
        Self {
            pack_mc_meta: PackMcMeta {
                information: PackInformation {
                    pack_format: Some(pack_format),
                    description: Value::String(description),
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
    pub fn new_pack(pack_mc_meta: PackMcMeta) -> Self {
        Self {
            pack_mc_meta,
            namespaces: HashMap::new(),
        }
    }

    pub fn write<P: AsRef<Path>>(&self, output: P) -> io::Result<()> {
        let output = output.as_ref();

        fs::create_dir_all(output)?;

        let mcmeta_path = output.join("pack.mcmeta");
        let mcmeta_content =
            serde_json::to_string_pretty(&self.pack_mc_meta).map_err(io::Error::other)?;
        fs::write(mcmeta_path, mcmeta_content)?;

        let data_path = output.join("data");

        for (name, namespace) in &self.namespaces {
            let namespace_path = data_path.join(name);

            namespace.write(&namespace_path)?;
        }

        Ok(())
    }

    pub fn get_namespace_mut(&mut self, name: &str) -> &mut DatapackNamespace {
        self.namespaces.entry_ref(name).or_default()
    }

    pub fn add_namespace(&mut self, name: &str, namespace: DatapackNamespace) {
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
