use crate::command::enums::team_collision_rule::TeamCollisionRule;
use crate::command::enums::team_color::TeamColor;
use crate::command::enums::team_visibility::TeamVisibility;
use crate::entity_selector::EntitySelector;
use crate::snbt::SNBT;
use minecraft_command_types_procederal_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TeamOption {
    DisplayName(SNBT),
    Color(TeamColor),
    FriendlyFire(bool),
    SeeFriendlyInvisibles(bool),
    NametagVisibility(TeamVisibility),
    DeathMessageVisibility(TeamVisibility),
    CollisionRule(TeamCollisionRule),
    Prefix(SNBT),
    Suffix(SNBT),
}

impl Display for TeamOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DisplayName(display_name) => {
                write!(f, "displayName {}", display_name)
            }
            Self::Color(color) => {
                write!(f, "color {}", color)
            }
            Self::FriendlyFire(friendly_fire) => {
                write!(f, "friendlyFire {}", friendly_fire)
            }
            Self::SeeFriendlyInvisibles(see_friendly_invisibles) => {
                write!(f, "seeFriendlyInvisibles {}", see_friendly_invisibles)
            }
            Self::NametagVisibility(visibility) => {
                write!(f, "nametagVisibility {}", visibility)
            }
            Self::DeathMessageVisibility(visibility) => {
                write!(f, "deathMessageVisibility {}", visibility)
            }
            Self::CollisionRule(collision_rule) => {
                write!(f, "collisionRule {}", collision_rule)
            }
            Self::Prefix(prefix) => {
                write!(f, "prefix {}", prefix)
            }
            Self::Suffix(suffix) => {
                write!(f, "suffix {}", suffix)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum TeamCommand {
    List(Option<String>),
    Add(String, Option<SNBT>),
    Remove(String),
    Empty(String),
    Join(String, Option<EntitySelector>),
    Leave(EntitySelector),
    Modify(String, TeamOption),
}

impl Display for TeamCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(name) => {
                f.write_str("list")?;

                if let Some(name) = name {
                    write!(f, " {}", name)?;
                }

                Ok(())
            }
            Self::Add(name, display_name) => {
                write!(f, "add {}", name)?;

                if let Some(display_name) = display_name {
                    write!(f, " {}", display_name)?;
                }

                Ok(())
            }
            Self::Remove(name) => {
                write!(f, "remove {}", name)
            }
            Self::Empty(name) => {
                write!(f, "empty {}", name)
            }
            Self::Join(name, selector) => {
                write!(f, "join {}", name)?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;
                }

                Ok(())
            }
            Self::Leave(selector) => {
                write!(f, "leave {}", selector)
            }
            Self::Modify(name, option) => {
                write!(f, "modify {} {}", name, option)
            }
        }
    }
}
