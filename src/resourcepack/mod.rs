use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use hashbrown::HashMap;
use serde_json::Value;

use crate::{
    pack_mc_meta::{PackMcMeta, pack_information::PackInformation},
    resourcepack::namespace::ResourcepackNamespace,
};

pub mod namespace;

#[derive(Debug, Clone)]
pub struct Resourcepack {
    pub pack_mc_mcmeta: PackMcMeta,
    pub namespaces: HashMap<String, ResourcepackNamespace>,
}

impl Resourcepack {
    #[must_use]
    pub fn new(pack_format: i32, description: String) -> Self {
        Self {
            pack_mc_mcmeta: PackMcMeta {
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
    pub fn open<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path.as_ref();

        let pack_mc_meta_file = File::open(path.join("pack.mcmeta")).ok()?;

        let pack_mc_meta =
            serde_json::from_reader::<_, PackMcMeta>(BufReader::new(pack_mc_meta_file)).ok()?;

        let assets_folder = path.join("assets");

        let namespace_entries = fs::read_dir(assets_folder).ok()?;

        let mut namespaces = HashMap::new();

        for namespace_entry in namespace_entries {
            let namespace_entry = namespace_entry.ok()?;

            let file_type = namespace_entry.file_type().ok()?;

            if !file_type.is_dir() {
                continue;
            }

            let namespace_folder_path = namespace_entry.path();

            let namespace_name = namespace_entry.file_name().into_string().ok()?;

            let namespace = ResourcepackNamespace::open(namespace_folder_path)?;

            namespaces.insert(namespace_name, namespace);
        }

        Some(Self {
            pack_mc_mcmeta: pack_mc_meta,
            namespaces,
        })
    }
}
