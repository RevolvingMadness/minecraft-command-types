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
pub mod stopwatch;
pub mod tag;
pub mod team;
pub mod teleport;
pub mod test;
pub mod tick;
pub mod time;
pub mod title;
pub mod trigger;
pub mod waypoint;
pub mod whitelist;
pub mod worldborder;

use crate::block::BlockState;
use crate::column_position::ColumnPosition;
use crate::command::advancement::AdvancementCommand;
use crate::command::attribute::AttributeCommand;
use crate::command::bossbar::BossbarCommand;
use crate::command::clone::CloneMaskMode;
use crate::command::damage::DamageType;
use crate::command::data::DataCommand;
use crate::command::datapack::DatapackCommand;
use crate::command::debug::DebugCommand;
use crate::command::dialog::DialogCommand;
use crate::command::effect::EffectCommand;
use crate::command::enums::setblock_mode::SetblockMode;
use crate::command::enums::sound_source::{SoundSource, StopSoundSource};
use crate::command::enums::weather_type::WeatherType;
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
use crate::command::random::RandomCommand;
use crate::command::recipe::{RecipeMode, RecipeType};
use crate::command::r#return::ReturnCommand;
use crate::command::ride::RideCommand;
use crate::command::rotate::RotateCommand;
use crate::command::schedule::ScheduleCommand;
use crate::command::scoreboard::ScoreboardCommand;
use crate::command::stopwatch::StopwatchCommand;
use crate::command::tag::TagCommand;
use crate::command::team::TeamCommand;
use crate::command::teleport::TeleportCommand;
use crate::command::test::TestCommand;
use crate::command::tick::TickCommand;
use crate::command::time::TimeCommand;
use crate::command::title::TitleCommand;
use crate::command::trigger::TriggerAction;
use crate::command::waypoint::WaypointCommand;
use crate::command::whitelist::WhitelistCommand;
use crate::command::worldborder::WorldborderCommand;
use crate::coordinate::{Coordinates, WorldCoordinate};
use crate::entity_selector::EntitySelector;
use crate::item::{ItemPredicate, ItemStack};
use crate::macroable::Macroable;
use crate::option_write_chain;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use crate::time::Time;
use crate::types::Float;
use enums::advancement_type::AdvancementType;
use enums::banlist_type::BanlistType;
use enums::clone_mode::CloneMode;
use enums::difficulty::Difficulty;
use enums::gamemode::Gamemode;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
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
        Float,
        Option<ResourceLocation>,
        Option<DamageType>,
    ),
    Data(DataCommand),
    Datapack(DatapackCommand),
    Debug(DebugCommand),
    DefaultGamemode(Gamemode),
    Deop(EntitySelector),
    Dialog(DialogCommand),
    Difficulty(Option<Difficulty>),
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
    Jfr(bool),
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
        Option<Float>,
        Option<Float>,
        Option<Float>,
    ),
    Publish(Option<bool>, Option<Gamemode>, Option<i32>),
    Random(RandomCommand),
    Recipe(RecipeMode, EntitySelector, RecipeType),
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
    SetWorldSpawn(Option<Coordinates>, Option<Float>),
    Spawnpoint(Option<EntitySelector>, Option<Coordinates>, Option<Float>),
    Spectate(Option<EntitySelector>, Option<EntitySelector>),
    SpreadPlayers(
        ColumnPosition,
        Float,
        Float,
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
    Stopwatch(StopwatchCommand),
    Summon(
        ResourceLocation,
        Option<Coordinates>,
        Option<Macroable<SNBT>>,
    ),
    Tag(EntitySelector, TagCommand),
    Team(TeamCommand),
    TeamMessage(String),
    Teleport(TeleportCommand),
    Tellraw(EntitySelector, SNBT),
    Test(TestCommand),
    Tick(TickCommand),
    Time(TimeCommand),
    Title(EntitySelector, TitleCommand),
    Transfer(String, Option<i32>, Option<EntitySelector>),
    Trigger(String, Option<TriggerAction>),
    Version,
    Waypoint(WaypointCommand),
    Weather(WeatherType, Option<Time>),
    Whitelist(WhitelistCommand),
    Worldborder(WorldborderCommand),
}

impl From<Command> for ExecuteSubcommand {
    fn from(value: Command) -> Self {
        Self::Run(Box::new(value))
    }
}

impl Command {
    pub const RETURN_VALUE_0: Self = Self::Return(ReturnCommand::VALUE_0);
    pub const RETURN_VALUE_1: Self = Self::Return(ReturnCommand::VALUE_1);
    pub const RETURN_FAIL: Self = Self::Return(ReturnCommand::FAIL);

    #[inline]
    #[must_use]
    pub fn run(self) -> ExecuteSubcommand {
        self.into()
    }

    #[must_use]
    pub fn get_permission_level(&self, is_multiplayer: bool) -> PermissionLevel {
        match self {
            Self::Help(..)
            | Self::List(..)
            | Self::Me(..)
            | Self::Message(..)
            | Self::Random(RandomCommand::ValueRoll(_, _, None) | RandomCommand::Reset(..))
            | Self::TeamMessage(..)
            | Self::Trigger(..) => PermissionLevel::try_from(0).unwrap(),
            Self::Advancement(..)
            | Self::Attribute(..)
            | Self::Bossbar(..)
            | Self::Clear(..)
            | Self::Clone {
                ..
            }
            | Self::Damage(..)
            | Self::Data(..)
            | Self::Datapack(..)
            | Self::DefaultGamemode(..)
            | Self::Dialog(..)
            | Self::Difficulty(..)
            | Self::Effect(..)
            | Self::Enchant(..)
            | Self::Execute(..)
            | Self::Experience(..)
            | Self::FetchProfile(..)
            | Self::Fill(..)
            | Self::FillBiome(..)
            | Self::Forceload(..)
            | Self::Function(..)
            | Self::Gamemode(..)
            | Self::Gamerule(..)
            | Self::Give(..)
            | Self::Item(..)
            | Self::Kill(..)
            | Self::Locate(..)
            | Self::Loot(..)
            | Self::Particle(..)
            | Self::Place(..)
            | Self::Playsound(..)
            | Self::Random(RandomCommand::ValueRoll(..))
            | Self::Recipe(..)
            | Self::Reload
            | Self::Return(..)
            | Self::Ride(..)
            | Self::Rotate(..)
            | Self::Say(..)
            | Self::Schedule(..)
            | Self::Scoreboard(..)
            | Self::Setblock(..)
            | Self::SetWorldSpawn(..)
            | Self::Spawnpoint(..)
            | Self::Spectate(..)
            | Self::SpreadPlayers(..)
            | Self::StopSound(..)
            | Self::Summon(..)
            | Self::Tag(..)
            | Self::Team(..)
            | Self::Teleport(..)
            | Self::Tellraw(..)
            | Self::Test(..)
            | Self::Time(..)
            | Self::Title(..)
            | Self::Version
            | Self::Waypoint(..)
            | Self::Weather(..)
            | Self::Worldborder(..) => PermissionLevel::try_from(2).unwrap(),
            Self::Ban(..)
            | Self::BanIP(..)
            | Self::Banlist(..)
            | Self::Debug(..)
            | Self::Deop(..)
            | Self::Kick(..)
            | Self::Op(..)
            | Self::Pardon(..)
            | Self::PardonIp(..)
            | Self::SetIdleTimeout(..)
            | Self::Tick(..)
            | Self::Transfer(..)
            | Self::Whitelist(..) => PermissionLevel::try_from(3).unwrap(),
            Self::Jfr(..)
            | Self::Perf(..)
            | Self::Publish(..)
            | Self::SaveAll(..)
            | Self::SaveOff
            | Self::SaveOn
            | Self::Stop
            | Self::Stopwatch(..) => PermissionLevel::try_from(4).unwrap(),
            Self::Seed => {
                let level = if is_multiplayer {
                    2
                } else {
                    0
                };
                PermissionLevel::try_from(level).unwrap()
            }
        }
    }

    #[must_use]
    pub const fn is_multiplayer_only(&self) -> bool {
        matches!(
            self,
            Self::Ban(..)
                | Self::BanIP(..)
                | Self::Banlist(..)
                | Self::Deop(..)
                | Self::Op(..)
                | Self::Pardon(..)
                | Self::PardonIp(..)
                | Self::Perf(..)
                | Self::SaveAll(..)
                | Self::SaveOff
                | Self::SaveOn
                | Self::SetIdleTimeout(..)
                | Self::Stop
                | Self::Transfer(..)
                | Self::Whitelist(..)
        )
    }

    #[must_use]
    pub fn has_side_effects(&self) -> bool {
        match self {
            Self::Advancement(..) => true,
            Self::Attribute(_, _, command) => command.has_side_effects(),
            Self::Ban(..) => true,
            Self::BanIP(..) => true,
            Self::Banlist(..) => false,
            Self::Bossbar(command) => command.has_side_effects(),
            Self::Clear(_, _, max_count) => max_count.is_none_or(|max_count| max_count != 0),
            Self::Clone {
                ..
            } => true,
            Self::Damage(_, amount, _, _) => *amount != 0.0,
            Self::Data(command) => command.has_side_effects(),
            Self::Datapack(command) => command.has_side_effects(),
            Self::Debug(..) => true,
            Self::DefaultGamemode(..) => true,
            Self::Deop(..) => true,
            Self::Dialog(..) => true,
            Self::Difficulty(difficulty) => difficulty.is_some(),
            Self::Effect(..) => true,
            Self::Enchant(..) => true,
            Self::Execute(subcommand) => subcommand.has_side_effects(),
            Self::Experience(command) => command.has_side_effects(),
            Self::FetchProfile(..) => false,
            Self::Fill(..) => true,
            Self::FillBiome(..) => true,
            Self::Forceload(command) => command.has_side_effects(),
            Self::Function(..) => true,
            Self::Gamemode(..) => true,
            Self::Gamerule(..) => true,
            Self::Give(..) => true,
            Self::Help(..) => false,
            Self::Item(..) => true,
            Self::Jfr(..) => true,
            Self::Kick(..) => true,
            Self::Kill(..) => true,
            Self::List(..) => false,
            Self::Locate(..) => false,
            Self::Loot(..) => true,
            Self::Me(..) => true,
            Self::Message(..) => true,
            Self::Op(..) => true,
            Self::Pardon(..) => true,
            Self::PardonIp(..) => true,
            Self::Particle(..) => true,
            Self::Perf(..) => true,
            Self::Place(..) => true,
            Self::Playsound(..) => true,
            Self::Publish(..) => true,
            Self::Random(..) => true,
            Self::Recipe(..) => true,
            Self::Reload => true,
            Self::Return(..) => true,
            Self::Ride(..) => true,
            Self::Rotate(..) => true,
            Self::SaveAll(..) => true,
            Self::SaveOff => true,
            Self::SaveOn => true,
            Self::Say(..) => true,
            Self::Schedule(..) => true,
            Self::Scoreboard(command) => command.has_side_effects(),
            Self::Seed => false,
            Self::Setblock(..) => true,
            Self::SetIdleTimeout(..) => true,
            Self::SetWorldSpawn(..) => true,
            Self::Spawnpoint(..) => true,
            Self::Spectate(..) => true,
            Self::SpreadPlayers(..) => true,
            Self::Stop => true,
            Self::StopSound(..) => true,
            Self::Stopwatch(command) => command.has_side_effects(),
            Self::Summon(..) => true,
            Self::Tag(_, command) => command.has_side_effects(),
            Self::Team(command) => command.has_side_effects(),
            Self::TeamMessage(..) => true,
            Self::Teleport(..) => true,
            Self::Tellraw(..) => true,
            Self::Test(..) => true,
            Self::Tick(command) => command.has_side_effects(),
            Self::Time(command) => command.has_side_effects(),
            Self::Title(..) => true,
            Self::Transfer(..) => true,
            Self::Trigger(..) => true,
            Self::Version => false,
            Self::Waypoint(command) => command.has_side_effects(),
            Self::Weather(..) => true,
            Self::Whitelist(command) => command.has_side_effects(),
            Self::Worldborder(command) => command.has_side_effects(),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advancement(type_, selector, command) => {
                write!(f, "advancement {} {} {}", type_, selector, command)
            }
            Self::Attribute(selector, attribute, command) => {
                write!(f, "attribute {} {} {}", selector, attribute, command)
            }
            Self::Ban(selectors, reason) => {
                write!(f, "ban {}", selectors)?;

                option_write_chain!(f, reason);

                Ok(())
            }
            Self::BanIP(target, reason) => {
                write!(f, "ban-ip {}", target)?;

                option_write_chain!(f, reason);

                Ok(())
            }
            Self::Banlist(type_) => {
                f.write_str("banlist")?;

                option_write_chain!(f, type_);

                Ok(())
            }
            Self::Bossbar(command) => write!(f, "bossbar {}", command),
            Self::Clear(selector, item, max_count) => {
                f.write_str("clear")?;

                option_write_chain!(f, selector, item, max_count);

                Ok(())
            }
            Self::Clone {
                source_dimension,
                begin,
                end,
                target_dimension,
                destination,
                strict,
                mask_mode,
                clone_mode,
            } => {
                f.write_str("clone")?;

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
            Self::Damage(target, amount, type_, command_type) => {
                write!(f, "damage {} {}", target, amount)?;

                option_write_chain!(f, type_, command_type);

                Ok(())
            }
            Self::Data(data_command) => write!(f, "data {}", data_command),
            Self::Datapack(datapack_command) => write!(f, "datapack {},", datapack_command),
            Self::Debug(debug_type) => write!(f, "debug {}", debug_type),
            Self::DefaultGamemode(gamemode) => write!(f, "defaultgamemode {}", gamemode),
            Self::Deop(selector) => write!(f, "deop {}", selector),
            Self::Dialog(dialog_command) => write!(f, "dialog {}", dialog_command),
            Self::Difficulty(difficulty) => {
                f.write_str("difficulty")?;

                option_write_chain!(f, difficulty);

                Ok(())
            }
            Self::Effect(effect_command) => write!(f, "effect {}", effect_command),
            Self::Enchant(selector, enchantment, level) => {
                write!(f, "enchant {} {}", selector, enchantment)?;

                option_write_chain!(f, level);

                Ok(())
            }
            Self::Execute(subcommand) => write!(f, "execute {}", subcommand),
            Self::Experience(command) => write!(f, "experience {}", command),
            Self::FetchProfile(command) => write!(f, "fetchprofile {}", command),
            Self::Fill(from, to, block_state, command) => {
                write!(f, "fill {} {} {}", from, to, block_state)?;

                option_write_chain!(f, command);

                Ok(())
            }
            Self::FillBiome(from, to, biome, filter) => {
                write!(f, "fillbiome {} {} {}", from, to, biome)?;

                option_write_chain!(f, filter);

                Ok(())
            }
            Self::Forceload(command) => write!(f, "forceload {}", command),
            Self::Function(function, arguments) => {
                write!(f, "function {}", function)?;

                option_write_chain!(f, arguments);

                Ok(())
            }
            Self::Gamemode(gamemode, selector) => {
                write!(f, "gamemode {}", gamemode)?;

                option_write_chain!(f, selector);

                Ok(())
            }
            Self::Gamerule(name, value) => {
                write!(f, "gamerule {}", name)?;

                option_write_chain!(f, value);

                Ok(())
            }
            Self::Give(selector, item, count) => {
                write!(f, "give {} {}", selector, item)?;

                option_write_chain!(f, count);

                Ok(())
            }
            Self::Help(command) => {
                f.write_str("help")?;

                option_write_chain!(f, command);

                Ok(())
            }
            Self::Item(source, slot, command) => {
                write!(f, "item {} {} {}", source, slot, command)
            }
            Self::Jfr(start) => {
                f.write_str("jfr ")?;

                if *start {
                    f.write_str("start")
                } else {
                    f.write_str("stop")
                }
            }
            Self::Kick(selector, reason) => {
                write!(f, "kick {}", selector)?;

                option_write_chain!(f, reason);

                Ok(())
            }
            Self::Kill(selector) => {
                f.write_str("kill")?;

                option_write_chain!(f, selector);

                Ok(())
            }
            Self::List(show_uuids) => {
                f.write_str("list")?;

                if *show_uuids {
                    f.write_str(" uuids")?;
                }

                Ok(())
            }
            Self::Locate(locate_type, id) => {
                write!(f, "locate {} {}", locate_type, id)
            }
            Self::Loot(target, source) => {
                write!(f, "loot {} {}", target, source)
            }
            Self::Me(message) => {
                write!(f, "me {}", message)
            }
            Self::Message(selector, message) => {
                write!(f, "msg {} {}", selector, message)
            }
            Self::Op(selector) => {
                write!(f, "op {}", selector)
            }
            Self::Pardon(selector) => {
                write!(f, "pardon {}", selector)
            }
            Self::PardonIp(selector) => {
                write!(f, "pardon-ip {}", selector)
            }
            Self::Particle(command) => {
                write!(f, "particle {}", command)
            }
            Self::Perf(start) => {
                f.write_str("perf ")?;

                if *start {
                    f.write_str("start")
                } else {
                    f.write_str("stop")
                }
            }
            Self::Place(command) => {
                write!(f, "place {}", command)
            }
            Self::Playsound(sound, source, selector, pos, volume, pitch, minimum_volume) => {
                write!(f, "playsound {}", sound)?;

                option_write_chain!(f, source, selector, pos, volume, pitch, minimum_volume);

                Ok(())
            }
            Self::Publish(allow_commands, gamemode, port) => {
                f.write_str("playsound")?;

                option_write_chain!(f, allow_commands, gamemode, port);

                Ok(())
            }
            Self::Random(command) => {
                write!(f, "random {}", command)
            }
            Self::Recipe(mode, selector, recipe_type) => {
                write!(f, "recipe {} {} {}", mode, selector, recipe_type)
            }
            Self::Reload => f.write_str("reload"),
            Self::Return(command) => {
                write!(f, "return {}", command)
            }
            Self::Ride(selector, command) => {
                write!(f, "ride {} {}", selector, command)
            }
            Self::Rotate(selector, command) => {
                write!(f, "rotate {} {}", selector, command)
            }
            Self::SaveAll(should_flush) => {
                f.write_str("save-all")?;

                if *should_flush {
                    f.write_str(" flush")?;
                }

                Ok(())
            }
            Self::SaveOff => f.write_str("save-off"),
            Self::SaveOn => f.write_str("save-on"),
            Self::Say(message) => {
                write!(f, "say {}", message)
            }
            Self::Schedule(command) => {
                write!(f, "schedule {}", command)
            }
            Self::Scoreboard(command) => {
                write!(f, "scoreboard {}", command)
            }
            Self::Seed => f.write_str("seed"),
            Self::Setblock(coordinates, block, mode) => {
                write!(f, "setblock {} {}", coordinates, block)?;

                option_write_chain!(f, mode);

                Ok(())
            }
            Self::SetIdleTimeout(minutes) => {
                write!(f, "setidletimeout {}", minutes)
            }
            Self::SetWorldSpawn(coordinates, angle) => {
                f.write_str("setworldspawn")?;

                option_write_chain!(f, coordinates, angle);

                Ok(())
            }
            Self::Spawnpoint(selector, coordinates, angle) => {
                f.write_str("spawnpoint")?;

                option_write_chain!(f, selector, coordinates, angle);

                Ok(())
            }
            Self::Spectate(target, player) => {
                f.write_str("spectate")?;

                option_write_chain!(f, target, player);

                Ok(())
            }
            Self::SpreadPlayers(
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

                option_write_chain!(f, max_height);

                write!(f, "{} {}", respect_teams, targets)
            }
            Self::Stop => f.write_str("stop"),
            Self::StopSound(selector, source, sound) => {
                write!(f, "stopsound {}", selector)?;

                option_write_chain!(f, source, sound);

                Ok(())
            }
            Self::Stopwatch(command) => {
                write!(f, "stopwatch {}", command)
            }
            Self::Summon(location, coordinates, snbt) => {
                write!(f, "summon {}", location)?;

                option_write_chain!(f, coordinates, snbt);

                Ok(())
            }
            Self::Tag(selector, command) => {
                write!(f, "tag {} {}", selector, command)
            }
            Self::Team(command) => {
                write!(f, "team {}", command)
            }
            Self::TeamMessage(message) => {
                write!(f, "teammsg {}", message)
            }
            Self::Teleport(command) => {
                write!(f, "teleport {}", command)
            }
            Self::Tellraw(selector, message) => {
                write!(f, "tellraw {} {}", selector, message)
            }
            Self::Test(command) => {
                write!(f, "test {}", command)
            }
            Self::Tick(command) => write!(f, "tick {}", command),
            Self::Time(command) => write!(f, "time {}", command),
            Self::Title(selector, command) => write!(f, "title {} {}", selector, command),
            Self::Transfer(hostname, port, selector) => {
                write!(f, "transfer {}", hostname)?;

                option_write_chain!(f, port, selector);

                Ok(())
            }
            Self::Trigger(objective, action) => {
                write!(f, "trigger {}", objective)?;

                option_write_chain!(f, action);

                Ok(())
            }
            Self::Version => f.write_str("version"),
            Self::Waypoint(command) => write!(f, "waypoint {}", command),
            Self::Weather(type_, duration) => {
                write!(f, "weather {}", type_)?;

                option_write_chain!(f, duration);

                Ok(())
            }
            Self::Whitelist(command) => write!(f, "whitelist {}", command),
            Self::Worldborder(command) => write!(f, "worldborder {}", command),
        }
    }
}
