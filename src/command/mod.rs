pub mod advancement;
pub mod attribute;
pub mod bossbar;
pub mod clone;
pub mod damage;
pub mod data;
pub mod datapack;
pub mod permission_level;

use crate::command::advancement::AdvancementCommand;
use crate::command::attribute::AttributeCommand;
use crate::command::bossbar::BossbarCommand;
use crate::command::clone::CloneMaskMode;
use crate::command::damage::DamageType;
use crate::command::data::DataCommand;
use crate::command::datapack::DatapackCommand;
use crate::command::permission_level::PermissionLevel;
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::enums::advancement_type::AdvancementType;
use crate::enums::banlist_type::BanlistType;
use crate::enums::clone_mode::CloneMode;
use crate::has_macro::HasMacro;
use crate::item::ItemPredicate;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum Command {
    Advancement(AdvancementType, EntitySelector, AdvancementCommand),
    Attribute(EntitySelector, ResourceLocation, AttributeCommand),
    Ban(EntitySelector, Option<String>),
    BanIP(String, Option<String>),
    Banlist(Option<BanlistType>),
    Bossbar(BossbarCommand),
    Clear(Option<EntitySelector>, Option<ItemPredicate>, Option<i32>),
    Clone {
        source_dimension: Option<ResourceLocation>,
        begin: Coordinates,
        end: Coordinates,
        target_dimension: Option<ResourceLocation>,
        destination: Coordinates,
        strict: bool,
        mask_mode: CloneMaskMode,
        clone_mode: CloneMode,
    },
    Damage(
        EntitySelector,
        NotNan<f32>,
        Option<ResourceLocation>,
        Option<DamageType>,
    ),
    Data(DataCommand),
    Datapack(DatapackCommand),
}

impl Command {
    pub fn get_permission_level(&self) -> PermissionLevel {
        match self {
            Command::Advancement(..)
            | Command::Attribute(..)
            | Command::Bossbar(..)
            | Command::Clear(..)
            | Command::Clone { .. }
            | Command::Damage(..)
            | Command::Data(..)
            | Command::Datapack(..) => PermissionLevel::try_from(2).unwrap(),
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
            Command::Clone {
                source_dimension,
                begin,
                end,
                target_dimension,
                destination,
                strict,
                mask_mode,
                clone_mode,
            } => {
                write!(f, "clone")?;

                if let Some(source_dimension) = source_dimension {
                    write!(f, " from {}", source_dimension)?;
                }

                write!(f, " {} {}", begin, end)?;

                if let Some(target_dimension) = target_dimension {
                    write!(f, " to {}", target_dimension)?;
                }

                write!(f, " {}", destination)?;

                if *strict {
                    write!(f, " strict")?;
                }

                write!(f, " {} {}", mask_mode, clone_mode)
            }
            Command::Damage(target, amount, type_, command_type) => {
                write!(f, "damage {} {}", target, amount)?;

                if let Some(type_) = type_ {
                    write!(f, " {}", type_)?;

                    if let Some(command_type) = command_type {
                        write!(f, " {}", command_type)?;
                    }
                }

                Ok(())
            }
            Command::Data(data_command) => write!(f, "data {}", data_command),
            Command::Datapack(datapack_command) => write!(f, "datapack {},", datapack_command),
        }
    }
}
