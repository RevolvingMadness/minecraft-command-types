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
pub mod particle_display_type;
pub mod random_type;
pub mod relation;
pub mod schedule_mode;
pub mod score_operation_operator;
pub mod scoreboard_criterion;
pub mod scoreboard_render_type;
pub mod setblock_mode;
pub mod sort;
pub mod sound_source;
pub mod store_type;
pub mod team_collision_rule;
pub mod team_color;
pub mod team_visibility;
pub mod template_mirror;
pub mod template_rotation;
pub mod time_of_day;
pub mod time_query_type;
pub mod title_type;

#[macro_export]
macro_rules! create_enum {
    (
        $name:ident,
        [ $($custom_derives:ident),* ],
        $($(#[$variant_attr:meta])* $variant:ident),+
        $(,)?
    ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, ::strum::Display, ::strum::EnumString, ::minecraft_command_types_proc_macros::HasMacro, ::serde::Serialize, ::serde::Deserialize, $($custom_derives),*)]
        #[strum(serialize_all = "snake_case")]
        pub enum $name {
            $($(#[$variant_attr])* $variant,)*
        }
    };

    (
        $serialize_all:expr,
        $name:ident,
        [ $($custom_derives:ident),* ],
        $($(#[$variant_attr:meta])* $variant:ident),+
        $(,)?
    ) => {
        #[derive(Debug, Clone, Eq, PartialEq, Hash, ::strum::Display, ::strum::EnumString, ::minecraft_command_types_proc_macros::HasMacro, ::serde::Serialize, ::serde::Deserialize, $($custom_derives),*)]
        #[strum(serialize_all = $serialize_all)]
        pub enum $name {
            $($(#[$variant_attr])* $variant,)*
        }
    };
}
