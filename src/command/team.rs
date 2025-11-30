use crate::command::enums::team_collision_rule::TeamCollisionRule;
use crate::command::enums::team_color::TeamColor;
use crate::command::enums::team_visibility::TeamVisibility;
use crate::entity_selector::EntitySelector;
use crate::snbt::SNBT;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
            TeamOption::DisplayName(display_name) => {
                write!(f, "displayName {}", display_name)
            }
            TeamOption::Color(color) => {
                write!(f, "color {}", color)
            }
            TeamOption::FriendlyFire(friendly_fire) => {
                write!(f, "friendlyFire {}", friendly_fire)
            }
            TeamOption::SeeFriendlyInvisibles(see_friendly_invisibles) => {
                write!(f, "seeFriendlyInvisibles {}", see_friendly_invisibles)
            }
            TeamOption::NametagVisibility(visibility) => {
                write!(f, "nametagVisibility {}", visibility)
            }
            TeamOption::DeathMessageVisibility(visibility) => {
                write!(f, "deathMessageVisibility {}", visibility)
            }
            TeamOption::CollisionRule(collision_rule) => {
                write!(f, "collisionRule {}", collision_rule)
            }
            TeamOption::Prefix(prefix) => {
                write!(f, "prefix {}", prefix)
            }
            TeamOption::Suffix(suffix) => {
                write!(f, "suffix {}", suffix)
            }
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
            TeamCommand::List(name) => {
                f.write_str("list")?;

                if let Some(name) = name {
                    write!(f, " {}", name)?;
                }

                Ok(())
            }
            TeamCommand::Add(name, display_name) => {
                write!(f, "add {}", name)?;

                if let Some(display_name) = display_name {
                    write!(f, " {}", display_name)?;
                }

                Ok(())
            }
            TeamCommand::Remove(name) => {
                write!(f, "remove {}", name)
            }
            TeamCommand::Empty(name) => {
                write!(f, "empty {}", name)
            }
            TeamCommand::Join(name, selector) => {
                write!(f, "join {}", name)?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;
                }

                Ok(())
            }
            TeamCommand::Leave(selector) => {
                write!(f, "leave {}", selector)
            }
            TeamCommand::Modify(name, option) => {
                write!(f, "modify {} {}", name, option)
            }
        }
    }
}
