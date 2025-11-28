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
pub mod function;
pub mod gamerule;
pub mod item;
pub mod item_source;
pub mod locate;
pub mod loot;
pub mod particle;
pub mod permission_level;
pub mod place;
pub mod random;
pub mod recipe;
pub mod r#return;
pub mod ride;
pub mod rotate;
pub mod schedule;
pub mod scoreboard;

use crate::block::BlockState;
use crate::column_position::ColumnPosition;
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
use crate::command::enums::setblock_mode::SetblockMode;
use crate::command::enums::sound_source::{SoundSource, StopSoundSource};
use crate::command::execute::ExecuteSubcommand;
use crate::command::experience::ExperienceCommand;
use crate::command::fetch_profile::FetchProfileCommand;
use crate::command::fill::FillCommand;
use crate::command::forceload::ForceloadCommand;
use crate::command::function::FunctionCommandArguments;
use crate::command::gamerule::GameruleValue;
use crate::command::item::ItemCommand;
use crate::command::item_source::ItemSource;
use crate::command::locate::LocateType;
use crate::command::loot::{LootSource, LootTarget};
use crate::command::particle::ParticleCommand;
use crate::command::permission_level::PermissionLevel;
use crate::command::place::PlaceCommand;
use crate::command::r#return::ReturnCommand;
use crate::command::random::RandomCommand;
use crate::command::recipe::RecipeType;
use crate::command::ride::RideCommand;
use crate::command::rotate::RotateCommand;
use crate::command::schedule::ScheduleCommand;
use crate::command::scoreboard::ScoreboardCommand;
use crate::coordinate::{Coordinates, WorldCoordinate};
use crate::entity_selector::EntitySelector;
use crate::item::{ItemPredicate, ItemStack};
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
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
    Function(ResourceLocation, Option<FunctionCommandArguments>),
    Gamemode(Gamemode, Option<EntitySelector>),
    Gamerule(String, Option<GameruleValue>),
    Give(EntitySelector, ItemStack, Option<i32>),
    Help(Option<String>),
    Item(ItemSource, String, ItemCommand),
    JFR(bool),
    Kick(EntitySelector, Option<String>),
    Kill(Option<EntitySelector>),
    List(bool),
    Locate(LocateType, ResourceLocation),
    Loot(LootTarget, LootSource),
    Me(String),
    Message(EntitySelector, String),
    Op(EntitySelector),
    Pardon(EntitySelector),
    PardonIp(String),
    Particle(ParticleCommand),
    Perf(bool),
    Place(PlaceCommand),
    Playsound(
        ResourceLocation,
        Option<SoundSource>,
        Option<EntitySelector>,
        Option<WorldCoordinate>,
        Option<NotNan<f32>>,
        Option<NotNan<f32>>,
        Option<NotNan<f32>>,
    ),
    Publish(Option<bool>, Option<Gamemode>, Option<i32>),
    Random(RandomCommand),
    Recipe(bool, EntitySelector, RecipeType),
    Reload,
    Return(ReturnCommand),
    Ride(EntitySelector, RideCommand),
    Rotate(EntitySelector, RotateCommand),
    SaveAll(bool),
    SaveOff,
    SaveOn,
    Say(String),
    Schedule(ScheduleCommand),
    Scoreboard(ScoreboardCommand),
    Seed,
    Setblock(Coordinates, BlockState, Option<SetblockMode>),
    SetIdleTimeout(i32),
    SetWorldSpawn(Option<Coordinates>, Option<NotNan<f32>>),
    Spawnpoint(
        Option<EntitySelector>,
        Option<Coordinates>,
        Option<NotNan<f32>>,
    ),
    Spectate(Option<EntitySelector>, Option<EntitySelector>),
    SpreadPlayers(
        ColumnPosition,
        NotNan<f32>,
        NotNan<f32>,
        Option<i32>,
        bool,
        EntitySelector,
    ),
    Stop,
    StopSound(
        EntitySelector,
        Option<StopSoundSource>,
        Option<ResourceLocation>,
    ),
    Summon(EntitySelector, Option<Coordinates>, Option<SNBT>),
    // Tag,
    // Team,
    // TeamMessage,
    // Teleport,
    // Tell,
    Tellraw(EntitySelector, SNBT),
    // Test,
    // Tick,
    // Time,
    // Title,
    // Tm,
    // Tp,
    // Transfer,
    // Trigger,
    // Version,
    // W,
    // Waypoint,
    // Weather,
    // Whitelist,
    // Worldborder,
    // Xp,
}

impl Command {
    pub fn get_permission_level(&self, is_multiplayer: bool) -> PermissionLevel {
        match self {
            Command::Help(..) | Command::List(..) | Command::Me(..) | Command::Message(..)
            | Command::Random(RandomCommand::ValueRoll(_, _, None))
            | Command::Random(RandomCommand::Reset(..))
            // | Command::TeamMessage(..)
            // | Command::Tell(..)
            // | Command::Tm(..)
            // | Command::Trigger(..)
            // | Command::W(..)
            => {
                PermissionLevel::try_from(0).unwrap()
            }
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
            | Command::Forceload(..)
            | Command::Function(..)
            | Command::Gamemode(..)
            | Command::Gamerule(..)
            | Command::Give(..)
            | Command::Item(..)
            | Command::Kill(..)
            | Command::Locate(..)
            | Command::Loot(..)
            | Command::Particle(..)
            | Command::Place(..)
            | Command::Playsound(..)
            | Command::Random(RandomCommand::ValueRoll(_, _, Some(_)))
            | Command::Recipe(..)
            | Command::Reload
            | Command::Return(..)
            | Command::Ride(..)
            | Command::Rotate(..)
            | Command::Say(..)
            | Command::Schedule(..)
            | Command::Scoreboard(..)
            | Command::Setblock(..)
            | Command::SetWorldSpawn(..)
            | Command::Spawnpoint(..)
            | Command::Spectate(..)
            | Command::SpreadPlayers(..)
            | Command::StopSound(..)
            | Command::Summon(..)
            // | Command::Tag(..)
            // | Command::Team(..)
            // | Command::Teleport(..)
            | Command::Tellraw(..)
            // | Command::Test(..)
            // | Command::Time(..)
            // | Command::Title(..)
            // | Command::Tp(..)
            // | Command::Version(..)
            // | Command::Waypoint(..)
            // | Command::Weather(..)
            // | Command::Worldborder(..)
            // | Command::Xp(..)
            => PermissionLevel::try_from(2).unwrap(),
            Command::Ban(..)
            | Command::BanIP(..)
            | Command::Banlist(..)
            | Command::Debug(..)
            | Command::Deop(..)
            | Command::Kick(..)
            | Command::Op(..)
            | Command::Pardon(..)
            | Command::PardonIp(..)
            | Command::SetIdleTimeout(..)
            // | Command::Tick(..)
            // | Command::Transfer(..)
            // | Command::Whitelist(..)
            => PermissionLevel::try_from(3).unwrap(),
            Command::JFR(..)
            | Command::Perf(..)
            | Command::Publish(..)
            | Command::SaveAll(..)
            | Command::SaveOff
            | Command::SaveOn
            | Command::Stop
            => PermissionLevel::try_from(4).unwrap(),
            Command::Seed => {
                let level = if is_multiplayer {2 } else { 0 };
                PermissionLevel::try_from(level).unwrap()
            }
        }
    }

    pub fn is_multiplayer_only(&self) -> bool {
        match self {
            Command::Ban(..)
            | Command::BanIP(..)
            | Command::Banlist(..)
            | Command::Deop(..)
            | Command::Op(..)
            | Command::Pardon(..)
            | Command::PardonIp(..)
            | Command::Perf(..)
            | Command::SaveAll(..)
            | Command::SaveOff
            | Command::SaveOn
            | Command::SetIdleTimeout(..)
            | Command::Stop => true,
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
            Command::Function(function, arguments) => {
                write!(f, "function {}", function)?;

                if let Some(arguments) = arguments {
                    write!(f, " {}", arguments)?;
                }

                Ok(())
            }
            Command::Gamemode(gamemode, selector) => {
                write!(f, "gamemode {}", gamemode)?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;
                }

                Ok(())
            }
            Command::Gamerule(name, value) => {
                write!(f, "gamerule {}", name)?;

                if let Some(value) = value {
                    write!(f, " {}", value)?;
                }

                Ok(())
            }
            Command::Give(selector, item, count) => {
                write!(f, "give {} {}", selector, item)?;

                if let Some(count) = count {
                    write!(f, " {}", count)?;
                }

                Ok(())
            }
            Command::Help(command) => {
                "help".fmt(f)?;

                if let Some(command) = command {
                    write!(f, " {}", command)?;
                }

                Ok(())
            }
            Command::Item(source, slot, command) => {
                write!(f, "item {} {} {}", source, slot, command)
            }
            Command::JFR(start) => {
                "jfr ".fmt(f)?;

                if *start {
                    write!(f, "start")?;
                } else {
                    write!(f, "stop")?;
                }

                Ok(())
            }
            Command::Kick(selector, reason) => {
                write!(f, "kick {}", selector)?;

                if let Some(reason) = reason {
                    write!(f, " {}", reason)?;
                }

                Ok(())
            }
            Command::Kill(selector) => {
                "kill".fmt(f)?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;
                }

                Ok(())
            }
            Command::List(show_uuids) => {
                "list".fmt(f)?;

                if *show_uuids {
                    " uuids".fmt(f)?;
                }

                Ok(())
            }
            Command::Locate(locate_type, id) => {
                write!(f, "locate {} {}", locate_type, id)
            }
            Command::Loot(target, source) => {
                write!(f, "loot {} {}", target, source)
            }
            Command::Me(message) => {
                write!(f, "me {}", message)
            }
            Command::Message(selector, message) => {
                write!(f, "msg {} {}", selector, message)
            }
            Command::Op(selector) => {
                write!(f, "op {}", selector)
            }
            Command::Pardon(selector) => {
                write!(f, "pardon {}", selector)
            }
            Command::PardonIp(selector) => {
                write!(f, "pardon-ip {}", selector)
            }
            Command::Particle(command) => {
                write!(f, "particle {}", command)
            }
            Command::Perf(start) => {
                write!(f, "perf ")?;

                if *start {
                    write!(f, "start")?;
                } else {
                    write!(f, "stop")?;
                }

                Ok(())
            }
            Command::Place(command) => {
                write!(f, "place {}", command)
            }
            Command::Playsound(sound, source, selector, pos, volume, pitch, minimum_volume) => {
                write!(f, "playsound {}", sound)?;

                if let Some(source) = source {
                    write!(f, " {}", source)?;

                    if let Some(selector) = selector {
                        write!(f, " {}", selector)?;

                        if let Some(pos) = pos {
                            write!(f, " {}", pos)?;

                            if let Some(volume) = volume {
                                write!(f, " {}", volume)?;

                                if let Some(pitch) = pitch {
                                    write!(f, " {}", pitch)?;

                                    if let Some(minimum_volume) = minimum_volume {
                                        write!(f, " {}", minimum_volume)?;
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
            Command::Publish(allow_commands, gamemode, port) => {
                "playsound".fmt(f)?;

                if let Some(allow_commands) = allow_commands {
                    write!(f, " {}", allow_commands)?;

                    if let Some(gamemode) = gamemode {
                        write!(f, " {}", gamemode)?;

                        if let Some(port) = port {
                            write!(f, " {}", port)?;
                        }
                    }
                }

                Ok(())
            }
            Command::Random(command) => {
                write!(f, "random {}", command)
            }
            Command::Recipe(give, selector, recipe_type) => {
                "recipe ".fmt(f)?;

                if *give {
                    "give".fmt(f)?;
                } else {
                    "take".fmt(f)?;
                }

                write!(f, " {} {}", selector, recipe_type)
            }
            Command::Reload => "reload".fmt(f),
            Command::Return(command) => {
                write!(f, "return {}", command)
            }
            Command::Ride(selector, command) => {
                write!(f, "ride {} {}", selector, command)
            }
            Command::Rotate(selector, command) => {
                write!(f, "rotate {} {}", selector, command)
            }
            Command::SaveAll(should_flush) => {
                "save-all".fmt(f)?;

                if *should_flush {
                    " flush".fmt(f)?;
                }

                Ok(())
            }
            Command::SaveOff => "save-off".fmt(f),
            Command::SaveOn => "save-on".fmt(f),
            Command::Say(message) => {
                write!(f, "say {}", message)
            }
            Command::Schedule(command) => {
                write!(f, "schedule {}", command)
            }
            Command::Scoreboard(command) => {
                write!(f, "scoreboard {}", command)
            }
            Command::Seed => "seed".fmt(f),
            Command::Setblock(coordinates, block, mode) => {
                write!(f, "setblock {} {}", coordinates, block)?;

                if let Some(mode) = mode {
                    write!(f, " {}", mode)?;
                }

                Ok(())
            }
            Command::SetIdleTimeout(minutes) => {
                write!(f, "setidletimeout {}", minutes)
            }
            Command::SetWorldSpawn(coordinates, angle) => {
                "setworldspawn".fmt(f)?;

                if let Some(coordinates) = coordinates {
                    write!(f, " {}", coordinates)?;

                    if let Some(angle) = angle {
                        write!(f, " {}", angle)?;
                    }
                }

                Ok(())
            }
            Command::Spawnpoint(selector, coordinates, angle) => {
                "spawnpoint".fmt(f)?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;
                    if let Some(coordinates) = coordinates {
                        write!(f, " {}", coordinates)?;

                        if let Some(angle) = angle {
                            write!(f, " {}", angle)?;
                        }
                    }
                }

                Ok(())
            }
            Command::Spectate(selector1, selector2) => {
                "spectate".fmt(f)?;

                if let Some(selector1) = selector1 {
                    write!(f, " {}", selector1)?;

                    if let Some(selector2) = selector2 {
                        write!(f, " {}", selector2)?;
                    }
                }

                Ok(())
            }
            Command::SpreadPlayers(
                center,
                spread_distance,
                max_range,
                max_height,
                respect_teams,
                targets,
            ) => {
                write!(
                    f,
                    "spreadplayers {} {} {} ",
                    center, spread_distance, max_range
                )?;

                if let Some(max_height) = max_height {
                    write!(f, "under {} ", max_height)?;
                }

                write!(f, "{} {}", respect_teams, targets)
            }
            Command::Stop => "stop".fmt(f),
            Command::StopSound(selector, source, sound) => {
                write!(f, "stopsound {}", selector)?;

                if let Some(source) = source {
                    write!(f, " {}", source)?;

                    if let Some(sound) = sound {
                        write!(f, " {}", sound)?;
                    }
                }

                Ok(())
            }
            Command::Summon(location, coordinates, snbt) => {
                write!(f, "summon {}", location)?;

                if let Some(coordinates) = coordinates {
                    write!(f, " {}", coordinates)?;

                    if let Some(snbt) = snbt {
                        write!(f, " {}", snbt)?;
                    }
                }

                Ok(())
            }
            // Command::Tag() => {}
            // Command::Team() => {}
            // Command::TeamMessage() => {}
            // Command::Teleport() => {}
            // Command::Tell() => {}
            Command::Tellraw(selector, message) => {
                write!(f, "tellraw {} {}", selector, message)
            } // Command::Test() => {}
              // Command::Tick() => {}
              // Command::Time() => {}
              // Command::Title() => {}
              // Command::Tm() => {}
              // Command::Tp() => {}
              // Command::Transfer() => {}
              // Command::Trigger() => {}
              // Command::Version() => {}
              // Command::W() => {}
              // Command::Waypoint() => {}
              // Command::Weather() => {}
              // Command::Whitelist() => {}
              // Command::Worldborder() => {}
              // Command::Xp() => {}
        }
    }
}
