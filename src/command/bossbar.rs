use crate::command::enums::bossbar_color::BossbarColor;
use crate::command::enums::bossbar_get_type::BossbarGetType;
use crate::command::enums::bossbar_style::BossbarStyle;
use crate::entity_selector::EntitySelector;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum BossbarSetType {
    Color(BossbarColor),
    Max(i32),
    Name(SNBT),
    Players(Option<EntitySelector>),
    Style(BossbarStyle),
    Value(i32),
    Visible(bool),
}

impl Display for BossbarSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Color(color) => write!(f, "color {}", color),
            Self::Max(max) => write!(f, "max {}", max),
            Self::Name(name) => write!(f, "name {}", name),
            Self::Players(players) => {
                f.write_str("players")?;

                if let Some(players) = players {
                    write!(f, " {}", players)?;
                }

                Ok(())
            }
            Self::Style(style) => write!(f, "style {}", style),
            Self::Value(value) => write!(f, "value {}", value),
            Self::Visible(visible) => write!(f, "visible {}", visible),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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
            Self::Add(id, name) => write!(f, "add {} {}", id, name),
            Self::Get(id, type_) => write!(f, "get {} {}", id, type_),
            Self::List => f.write_str("list"),
            Self::Remove(id) => write!(f, "remove {}", id),
            Self::Set(id, set_type) => write!(f, "set {} {}", id, set_type),
        }
    }
}
