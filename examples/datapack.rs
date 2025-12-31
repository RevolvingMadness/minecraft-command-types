use minecraft_command_types::datapack::pack::Pack;
use minecraft_command_types::datapack::pack::format::Format;
use minecraft_command_types::datapack::tag::{Tag, TagType, TagValue};
use minecraft_command_types::datapack::{Datapack, Namespace, PackMCMeta};
use minecraft_command_types::resource_location::ResourceLocation;
use nonempty::nonempty;

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

    my_namespace.add_function(&nonempty!["main".to_string()], "say Datapack loaded!");

    my_namespace.add_function(
        &nonempty!["utils".to_string(), "teleport".to_string()],
        "tp @s ~ ~10 ~",
    );

    my_namespace.add_tag(
        TagType::Block,
        &nonempty!["cool_blocks".to_string()],
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
    );

    let mut my_datapack = Datapack {
        pack: pack_meta,
        namespaces: BTreeMap::new(),
    };

    my_datapack.add_namespace("mydp", my_namespace);

    let output_path = Path::new("my_awesome_datapack");
    if let Err(e) = my_datapack.write(output_path) {
        eprintln!("Failed to write datapack: {}", e);
    }
}
