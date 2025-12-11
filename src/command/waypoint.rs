use crate::command::enums::team_color_with_reset::TeamColorWithReset;
use crate::entity_selector::EntitySelector;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum WaypointColor {
    Color(TeamColorWithReset),
    Hex(String),
    Reset,
}

impl Display for WaypointColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WaypointColor::Color(color) => color.fmt(f),
            WaypointColor::Hex(hex) => write!(f, "hex {}", hex),
            WaypointColor::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum WaypointStyleModification {
    Set(ResourceLocation),
    Reset,
}

impl Display for WaypointStyleModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WaypointStyleModification::Set(style) => write!(f, "set {}", style),
            WaypointStyleModification::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum WaypointModification {
    Color(WaypointColor),
    Style(WaypointStyleModification),
}

impl Display for WaypointModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WaypointModification::Color(color) => write!(f, "color {}", color),
            WaypointModification::Style(style) => write!(f, "style {}", style),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum WaypointCommand {
    List,
    Modify(EntitySelector, WaypointModification),
}

impl Display for WaypointCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WaypointCommand::List => f.write_str("list"),
            WaypointCommand::Modify(selector, modification) => {
                write!(f, "modify {} {}", selector, modification)
            }
        }
    }
}
