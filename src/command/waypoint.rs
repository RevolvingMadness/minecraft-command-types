use crate::command::{Command, enums::team_color_with_reset::TeamColorWithReset};
use crate::entity_selector::EntitySelector;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WaypointColor {
    Color(TeamColorWithReset),
    Hex(String),
    Reset,
}

impl Display for WaypointColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Color(color) => color.fmt(f),
            Self::Hex(hex) => write!(f, "hex {}", hex),
            Self::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WaypointStyleModification {
    Set(ResourceLocation),
    Reset,
}

impl Display for WaypointStyleModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set(style) => write!(f, "set {}", style),
            Self::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WaypointModification {
    Color(WaypointColor),
    Style(WaypointStyleModification),
}

impl Display for WaypointModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Color(color) => write!(f, "color {}", color),
            Self::Style(style) => write!(f, "style {}", style),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WaypointCommand {
    List,
    Modify(EntitySelector, WaypointModification),
}

impl Display for WaypointCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List => f.write_str("list"),
            Self::Modify(selector, modification) => {
                write!(f, "modify {} {}", selector, modification)
            }
        }
    }
}

impl From<WaypointCommand> for Command {
    fn from(value: WaypointCommand) -> Self {
        Self::Waypoint(value)
    }
}
