use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::resourcepack::block_state_definition::BlockstateDefinition;

#[derive(Debug, Clone)]
pub struct ResourcepackNamespace {
    pub blockstates: HashMap<String, BlockstateDefinition>,
    // pub items: HashMap<String, ItemModelDefinition>,
}

impl ResourcepackNamespace {
    fn open_registry<Definition: DeserializeOwned>(
        definitions_folder_path: PathBuf,
    ) -> Option<HashMap<String, Definition>> {
        let definition_entries = fs::read_dir(definitions_folder_path).ok()?;

        let mut definitions = HashMap::new();

        for definition_entry in definition_entries {
            let definitions_definition_entry = definition_entry.ok()?;

            let file_type = definitions_definition_entry.file_type().ok()?;

            if !file_type.is_file() {
                continue;
            }

            let definitions_definition_file_path = definitions_definition_entry.path();

            let definition_name = definitions_definition_file_path
                .file_stem()?
                .to_str()?
                .to_owned();

            let definitions_definition_file = File::open(definitions_definition_file_path).ok()?;

            let definitions_definition = serde_json::from_reader::<_, Definition>(BufReader::new(
                definitions_definition_file,
            ))
            .unwrap();

            definitions.insert(definition_name, definitions_definition);
        }

        Some(definitions)
    }

    #[must_use]
    pub fn open<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path.as_ref();

        let blockstates = Self::open_registry(path.join("blockstates"))?;
        // let items = Self::open_registry(path.join("items"))?;

        Some(Self {
            blockstates,
            // items
        })
    }
}
