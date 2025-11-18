use minecraft_command_types::datapack::pack::Pack;
use minecraft_command_types::datapack::pack::format::Format;
use minecraft_command_types::datapack::tag::{Tag, TagType, TagValue};
use minecraft_command_types::datapack::{Datapack, FilePathNode, Namespace, PackMCMeta};
use minecraft_command_types::resource_location::ResourceLocation;

fn main() {
    use std::collections::BTreeMap;
    use std::path::Path;

    let pack_meta = PackMCMeta {
        pack: Pack {
            description: serde_json::json!("A datapack written with Rust!"),
            pack_format: Some(15),
            supported_formats: Some(Format::Array(15, 20)),
            min_format: None,
            max_format: None,
        },
        features: None,
        filter: None,
        overlays: None,
        language: None,
    };

    let mut my_namespace = Namespace::default();

    my_namespace.functions.push(FilePathNode::File(
        "main".to_string(),
        "say Datapack loaded!".to_string(),
    ));

    my_namespace.functions.push(FilePathNode::Directory(
        "utils".to_string(),
        vec![FilePathNode::File(
            "teleport".to_string(),
            "tp @s ~ ~10 ~".to_string(),
        )],
    ));

    let mut block_tags = BTreeMap::new();
    block_tags.insert(
        TagType::Block,
        vec![FilePathNode::File(
            "cool_blocks".to_string(),
            Tag {
                replace: Some(false),
                values: vec![
                    TagValue::ResourceLocation(ResourceLocation::new_namespace_path(
                        "minecraft",
                        "diamond_block",
                    )),
                    TagValue::ResourceLocation(ResourceLocation::new_namespace_path(
                        "minecraft",
                        "emerald_block",
                    )),
                ],
            },
        )],
    );
    my_namespace.tags = block_tags;

    let mut namespaces = BTreeMap::new();
    namespaces.insert("mydp".to_string(), my_namespace);

    let my_datapack = Datapack {
        pack: pack_meta,
        namespaces,
    };

    let output_path = Path::new("my_awesome_datapack");
    if let Err(e) = my_datapack.write(output_path) {
        eprintln!("Failed to write datapack: {}", e);
    }
}
