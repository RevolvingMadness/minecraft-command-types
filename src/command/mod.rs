pub mod advancement;
pub mod attribute;
pub mod bossbar;
pub mod clone;
pub mod damage;
pub mod data;
pub mod datapack;
pub mod debug;
pub mod dialog;
pub mod effect;
pub mod enums;
pub mod execute;
pub mod experience;
pub mod fetch_profile;
pub mod fill;
pub mod forceload;
pub mod permission_level;

use crate::block::BlockState;
use crate::command::advancement::AdvancementCommand;
use crate::command::attribute::AttributeCommand;
use crate::command::bossbar::BossbarCommand;
use crate::command::clone::CloneMaskMode;
use crate::command::damage::DamageType;
use crate::command::data::DataCommand;
use crate::command::datapack::DatapackCommand;
use crate::command::debug::DebugCommandType;
use crate::command::dialog::DialogCommand;
use crate::command::effect::EffectCommand;
use crate::command::execute::ExecuteSubcommand;
use crate::command::experience::ExperienceCommand;
use crate::command::fetch_profile::FetchProfileCommand;
use crate::command::fill::FillCommand;
use crate::command::forceload::ForceloadCommand;
use crate::command::permission_level::PermissionLevel;
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::has_macro::HasMacro;
use crate::item::ItemPredicate;
use crate::resource_location::ResourceLocation;
use enums::advancement_type::AdvancementType;
use enums::banlist_type::BanlistType;
use enums::clone_mode::CloneMode;
use enums::difficulty::Difficulty;
use enums::gamemode::Gamemode;
use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct PlayerScore {
    pub selector: EntitySelector,
    pub objective: String,
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.selector, self.objective)
    }
}

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
    Debug(DebugCommandType),
    DefaultGamemode(Gamemode),
    Deop(EntitySelector),
    Dialog(DialogCommand),
    Difficulty(Difficulty),
    Effect(EffectCommand),
    Enchant(EntitySelector, ResourceLocation, Option<i32>),
    Execute(ExecuteSubcommand),
    Experience(ExperienceCommand),
    FetchProfile(FetchProfileCommand),
    Fill(Coordinates, Coordinates, BlockState, Option<FillCommand>),
    FillBiome(
        Coordinates,
        Coordinates,
        ResourceLocation,
        Option<ResourceLocation>,
    ),
    Forceload(ForceloadCommand),
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
            | Command::Datapack(..)
            | Command::DefaultGamemode(..)
            | Command::Dialog(..)
            | Command::Difficulty(..)
            | Command::Effect(..)
            | Command::Enchant(..)
            | Command::Execute(..)
            | Command::Experience(..)
            | Command::FetchProfile(..)
            | Command::Fill(..)
            | Command::FillBiome(..)
            | Command::Forceload(..) => PermissionLevel::try_from(2).unwrap(),
            Command::Ban(..)
            | Command::BanIP(..)
            | Command::Banlist(..)
            | Command::Debug(..)
            | Command::Deop(..) => PermissionLevel::try_from(3).unwrap(),
        }
    }

    pub fn is_multiplayer_only(&self) -> bool {
        match self {
            Command::Ban(..) | Command::BanIP(..) | Command::Banlist(..) | Command::Deop(..) => {
                true
            }
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
            Command::Debug(debug_type) => write!(f, "debug {}", debug_type),
            Command::DefaultGamemode(gamemode) => write!(f, "defaultgamemode {}", gamemode),
            Command::Deop(selector) => write!(f, "deop {}", selector),
            Command::Dialog(dialog_command) => write!(f, "dialog {}", dialog_command),
            Command::Difficulty(difficulty) => write!(f, "difficulty {}", difficulty),
            Command::Effect(effect_command) => write!(f, "effect {}", effect_command),
            Command::Enchant(selector, enchantment, level) => {
                write!(f, "enchant {} {}", selector, enchantment)?;

                if let Some(level) = level {
                    write!(f, " {}", level)?;
                }

                Ok(())
            }
            Command::Execute(subcommand) => write!(f, "execute {}", subcommand),
            Command::Experience(command) => write!(f, "experience {}", command),
            Command::FetchProfile(command) => write!(f, "fetchprofile {}", command),
            Command::Fill(from, to, block_state, command) => {
                write!(f, "fill {} {} {}", from, to, block_state)?;

                if let Some(command) = command {
                    write!(f, " {}", command)?;
                }

                Ok(())
            }
            Command::FillBiome(from, to, biome, filter) => {
                write!(f, "fillbiome {} {} {}", from, to, biome)?;

                if let Some(filter) = filter {
                    write!(f, " {}", filter)?;
                }

                Ok(())
            }
            Command::Forceload(command) => write!(f, "forceload {}", command),
        }
    }
}
