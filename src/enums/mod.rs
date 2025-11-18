pub mod advancement_type;
pub mod attribute;
pub mod axis;
pub mod banlist_type;
pub mod bossbar_color;
pub mod bossbar_get_type;
pub mod bossbar_store_type;
pub mod bossbar_style;
pub mod clone_mode;
pub mod datapack_list_type;
pub mod difficulty;
pub mod entity_anchor;
pub mod experience_type;
pub mod fill_mode;
pub mod fill_replace_mode;
pub mod gamemode;
pub mod heightmap;
pub mod if_blocks_mode;
pub mod numeric_snbt_type;
pub mod relation;
pub mod sort;
pub mod store_type;

#[macro_export]
macro_rules! create_enum {
    (
        $name:ident,
        [ $($custom_derives:ident),* ],
        $($(#[$variant_attr:meta])* $variant:ident),+
        $(,)?
    ) => {
        use crate::has_macro::HasMacro;
        use minecraft_command_types_proc_macros::HasMacro;
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};
        use strum::{Display, EnumString};

        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
        #[derive(Debug, Clone, Eq, PartialEq, Hash, Display, EnumString, HasMacro, $($custom_derives),*)]
        #[strum(serialize_all = "snake_case")]
        pub enum $name {
            $($(#[$variant_attr])* $variant,)*
        }
    };
}
