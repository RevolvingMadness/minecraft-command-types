pub mod advancement;
pub mod attribute;
pub mod bossbar;
pub mod permission_level;

use crate::command::advancement::AdvancementCommand;
use crate::command::attribute::AttributeCommand;
use crate::command::bossbar::BossbarCommand;
use crate::command::permission_level::PermissionLevel;
use crate::entity_selector::EntitySelector;
use crate::enums::advancement_type::AdvancementType;
use crate::enums::banlist_type::BanlistType;
use crate::item::ItemPredicate;
use crate::resource_location::ResourceLocation;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    Advancement(AdvancementType, EntitySelector, AdvancementCommand),
    Attribute(EntitySelector, ResourceLocation, AttributeCommand),
    Ban(EntitySelector, Option<String>),
    BanIP(String, Option<String>),
    Banlist(Option<BanlistType>),
    Bossbar(BossbarCommand),
    Clear(Option<EntitySelector>, Option<ItemPredicate>, Option<i32>),
}

impl Command {
    pub fn get_permission_level(&self) -> PermissionLevel {
        match self {
            Command::Advancement(..)
            | Command::Attribute(..)
            | Command::Bossbar(..)
            | Command::Clear(..) => PermissionLevel::try_from(2).unwrap(),
            Command::Ban(..) | Command::BanIP(..) | Command::Banlist(..) => {
                PermissionLevel::try_from(3).unwrap()
            }
        }
    }

    pub fn is_multiplayer_only(&self) -> bool {
        match self {
            Command::Ban(..) | Command::BanIP(..) | Command::Banlist(..) => true,
            _ => false,
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Advancement(type_, selector, command) => {
                write!(f, "advancement {} {} {}", type_, selector, command)
            }
            Command::Attribute(selector, attribute, command) => {
                write!(f, "attribute {} {} {}", selector, attribute, command)
            }
            Command::Ban(selectors, reason) => {
                write!(f, "ban {}", selectors)?;

                if let Some(reason) = reason {
                    write!(f, " {}", reason)?;
                }

                Ok(())
            }
            Command::BanIP(target, reason) => {
                write!(f, "ban-ip {}", target)?;

                if let Some(reason) = reason {
                    write!(f, " {}", reason)?;
                }

                Ok(())
            }
            Command::Banlist(type_) => {
                write!(f, "banlist")?;

                if let Some(type_) = type_ {
                    write!(f, " {}", type_)?;
                }

                Ok(())
            }
            Command::Bossbar(command) => write!(f, "bossbar {}", command),
            Command::Clear(selector, item, max_count) => {
                "clear".fmt(f)?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;

                    if let Some(item) = item {
                        write!(f, " {}", item)?;

                        if let Some(max_count) = max_count {
                            write!(f, " {}", max_count)?;
                        }
                    }
                }

                Ok(())
            }
        }
    }
}
