use crate::command::enums::team_color::TeamColor;
use crate::command::enums::team_visibility::TeamVisibility;
use crate::command::{Command, enums::team_collision_rule::TeamCollisionRule};
use crate::entity_selector::EntitySelector;
use crate::option_write_chain;
use crate::snbt::SNBT;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, HasMacro)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(name) => {
                f.write_str("list")?;

                option_write_chain!(f, name);

                Ok(())
            }
            Self::Add(name, display_name) => {
                write!(f, "add {}", name)?;

                option_write_chain!(f, display_name);

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

                option_write_chain!(f, selector);

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

impl From<TeamCommand> for Command {
    fn from(value: TeamCommand) -> Self {
        Self::Team(value)
    }
}

impl TeamCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::List(..) => false,
            Self::Add(..) => true,
            Self::Remove(..) => true,
            Self::Empty(..) => true,
            Self::Join(..) => true,
            Self::Leave(..) => true,
            Self::Modify(..) => true,
        }
    }
}
