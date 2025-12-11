use crate::command::enums::bossbar_color::BossbarColor;
use crate::command::enums::bossbar_get_type::BossbarGetType;
use crate::command::enums::bossbar_style::BossbarStyle;
use crate::entity_selector::EntitySelector;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum BossbarSetType {
    /// Set the text color (if no color was specified as part of a text component) and bar color. Defaults to `white` upon creation.
    Color(BossbarColor),
    /// Set the bossbar's maximum value. Defaults to `100` upon creation.
    Max(i32),
    /// Set the bossbar's name.
    Name(SNBT),
    /// Change the set of players to whom the bar is visible. Defaults to none upon creation.
    Players(Option<EntitySelector>),
    /// Set the bossbar's visual amount of segments: continuous, 6 segments, 10 segments, 12 segments, or 20 segments. Defaults to `progress` upon creation.
    Style(BossbarStyle),
    /// Set the bossbar's current value. Defaults to `0` upon creation.
    Value(i32),
    /// Set the bossbar's visibility. Defaults to `true` upon creation.
    Visible(bool),
}

impl Display for BossbarSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BossbarSetType::Color(color) => write!(f, "color {}", color),
            BossbarSetType::Max(max) => write!(f, "max {}", max),
            BossbarSetType::Name(name) => write!(f, "name {}", name),
            BossbarSetType::Players(players) => {
                f.write_str("players")?;

                if let Some(players) = players {
                    write!(f, " {}", players)?;
                }

                Ok(())
            }
            BossbarSetType::Style(style) => write!(f, "style {}", style),
            BossbarSetType::Value(value) => write!(f, "value {}", value),
            BossbarSetType::Visible(visible) => write!(f, "visible {}", visible),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum BossbarCommand {
    Add(ResourceLocation, SNBT),
    Get(ResourceLocation, BossbarGetType),
    List,
    Remove(ResourceLocation),
    Set(ResourceLocation, BossbarSetType),
}

impl Display for BossbarCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BossbarCommand::Add(id, name) => write!(f, "add {} {}", id, name),
            BossbarCommand::Get(id, type_) => write!(f, "get {} {}", id, type_),
            BossbarCommand::List => f.write_str("list"),
            BossbarCommand::Remove(id) => write!(f, "remove {}", id),
            BossbarCommand::Set(id, set_type) => write!(f, "set {} {}", id, set_type),
        }
    }
}
