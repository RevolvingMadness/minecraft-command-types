use crate::block::BlockState;
use crate::column_position::ColumnPosition;
use crate::command::data::DataTarget;
use crate::command::enums::axis::Axis;
use crate::command::enums::bossbar_store_type::BossbarStoreType;
use crate::command::enums::entity_anchor::EntityAnchor;
use crate::command::enums::heightmap::Heightmap;
use crate::command::enums::if_blocks_mode::IfBlocksMode;
use crate::command::enums::numeric_snbt_type::NumericSNBTType;
use crate::command::enums::relation::Relation;
use crate::command::enums::store_type::StoreType;
use crate::command::item_source::ItemSource;
use crate::command::{Command, PlayerScore};
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::item::ItemPredicate;
use crate::nbt_path::NbtPath;
use crate::range::{FloatRange, IntegerRange};
use crate::resource_location::ResourceLocation;
use crate::rotation::Rotation;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use strum::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum Facing {
    Position(Coordinates),
    Entity(EntitySelector, EntityAnchor),
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Facing::Position(coords) => coords.fmt(f),
            Facing::Entity(selector, anchor) => write!(f, "entity {} {}", selector, anchor),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum Positioned {
    Position(Coordinates),
    As(EntitySelector),
    Over(Heightmap),
}

impl Display for Positioned {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Positioned::Position(coords) => coords.fmt(f),
            Positioned::As(selector) => write!(f, "as {}", selector),
            Positioned::Over(heightmap) => write!(f, "over {}", heightmap),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum Rotated {
    Rotation(Rotation),
    As(EntitySelector),
}

impl Display for Rotated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotated::Rotation(rotation) => rotation.fmt(f),
            Rotated::As(selector) => write!(f, "as {}", selector),
        }
    }
}

#[derive(Display, Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScoreComparisonOperator {
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = "<=")]
    LessThanOrEqualTo,
    #[strum(serialize = "=")]
    EqualTo,
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = ">=")]
    GreaterThanOrEqualTo,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScoreComparison {
    Range(IntegerRange),
    Score(ScoreComparisonOperator, PlayerScore),
}

impl Display for ScoreComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreComparison::Range(range) => write!(f, "matches {}", range),
            ScoreComparison::Score(operator, right) => {
                write!(f, "{} {}", operator, right)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ExecuteIfSubcommand {
    Biome(
        Coordinates,
        ResourceLocation,
        Option<Box<ExecuteSubcommand>>,
    ),
    Block(Coordinates, BlockState, Option<Box<ExecuteSubcommand>>),
    Blocks(
        Coordinates,
        Coordinates,
        Coordinates,
        IfBlocksMode,
        Option<Box<ExecuteSubcommand>>,
    ),
    Data(DataTarget, NbtPath, Option<Box<ExecuteSubcommand>>),
    Dimension(ResourceLocation, Option<Box<ExecuteSubcommand>>),
    Entity(EntitySelector, Option<Box<ExecuteSubcommand>>),
    Function(ResourceLocation, Box<ExecuteSubcommand>),
    Items(
        ItemSource,
        String,
        ItemPredicate,
        Option<Box<ExecuteSubcommand>>,
    ),
    Loaded(ColumnPosition, Option<Box<ExecuteSubcommand>>),
    Predicate(ResourceLocation, Option<Box<ExecuteSubcommand>>),
    Score(PlayerScore, ScoreComparison, Option<Box<ExecuteSubcommand>>),
    Stopwatch(ResourceLocation, FloatRange),
}

impl Display for ExecuteIfSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecuteIfSubcommand::Biome(coords, id, next) => {
                write!(f, "biome {} {}", coords, id)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Block(coords, predicate, next) => {
                write!(f, "block {} {}", coords, predicate)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Blocks(start, end, dest, mode, next) => {
                write!(f, "blocks {} {} {} {}", start, end, dest, mode)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Data(target, path, next) => {
                write!(f, "data {} {}", target, path)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Dimension(id, next) => {
                write!(f, "dimension {}", id)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Entity(selector, next) => {
                write!(f, "entity {}", selector)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Function(id, next) => {
                write!(f, "function {} {}", id, next)
            }
            ExecuteIfSubcommand::Items(source, slot, predicate, next) => {
                write!(f, "items {} {} {}", source, slot, predicate)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Loaded(coords, next) => {
                write!(f, "loaded {}", coords)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Predicate(id, next) => {
                write!(f, "predicate {}", id)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Score(score, comparison, next) => {
                write!(f, "score {} {}", score, comparison)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            ExecuteIfSubcommand::Stopwatch(location, range) => {
                write!(f, "stopwatch {} {}", location, range)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ExecuteStoreSubcommand {
    Block(
        Coordinates,
        NbtPath,
        NumericSNBTType,
        NotNan<f32>,
        Box<ExecuteSubcommand>,
    ),
    Bossbar(ResourceLocation, BossbarStoreType, Box<ExecuteSubcommand>),
    Entity(
        EntitySelector,
        NbtPath,
        NumericSNBTType,
        NotNan<f32>,
        Box<ExecuteSubcommand>,
    ),
    Score(PlayerScore, Box<ExecuteSubcommand>),
    Storage(
        ResourceLocation,
        NbtPath,
        NumericSNBTType,
        NotNan<f32>,
        Box<ExecuteSubcommand>,
    ),
}

impl Display for ExecuteStoreSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecuteStoreSubcommand::Block(coords, path, num_type, scale, next) => {
                write!(
                    f,
                    "block {} {} {} {} {}",
                    coords, path, num_type, scale, next
                )?;

                Ok(())
            }
            ExecuteStoreSubcommand::Bossbar(id, store_type, next) => {
                write!(f, "bossbar {} {} {}", id, store_type, next)?;

                Ok(())
            }
            ExecuteStoreSubcommand::Entity(selector, path, num_type, scale, next) => {
                write!(
                    f,
                    "entity {} {} {} {} {}",
                    selector, path, num_type, scale, next
                )?;

                Ok(())
            }
            ExecuteStoreSubcommand::Score(score, next) => {
                write!(f, "score {} {}", score, next)?;

                Ok(())
            }
            ExecuteStoreSubcommand::Storage(id, path, num_type, scale, next) => {
                write!(f, "storage {} {} {} {} {}", id, path, num_type, scale, next)?;

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ExecuteSubcommand {
    Align(BTreeSet<Axis>, Box<ExecuteSubcommand>),
    Anchored(EntityAnchor, Box<ExecuteSubcommand>),
    As(EntitySelector, Box<ExecuteSubcommand>),
    At(EntitySelector, Box<ExecuteSubcommand>),
    Facing(Facing, Box<ExecuteSubcommand>),
    In(ResourceLocation, Box<ExecuteSubcommand>),
    On(Relation, Box<ExecuteSubcommand>),
    Positioned(Positioned, Box<ExecuteSubcommand>),
    Rotated(Rotated, Box<ExecuteSubcommand>),
    Summon(ResourceLocation, Box<ExecuteSubcommand>),
    If(bool, ExecuteIfSubcommand),
    Store(StoreType, ExecuteStoreSubcommand),
    Run(Box<Command>),
}

impl Display for ExecuteSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecuteSubcommand::Align(axes, next) => {
                let axes_str: String = axes.iter().map(|a| a.to_string()).collect();
                write!(f, "align {} {}", axes_str, next)?;

                Ok(())
            }
            ExecuteSubcommand::Anchored(anchor, next) => {
                write!(f, "anchored {} {}", anchor, next)?;

                Ok(())
            }
            ExecuteSubcommand::As(selector, next) => {
                write!(f, "as {} {}", selector, next)?;

                Ok(())
            }
            ExecuteSubcommand::At(selector, next) => {
                write!(f, "at {} {}", selector, next)?;

                Ok(())
            }
            ExecuteSubcommand::Facing(facing, next) => {
                write!(f, "facing {} {}", facing, next)?;

                Ok(())
            }
            ExecuteSubcommand::In(dimension, next) => {
                write!(f, "in {} {}", dimension, next)?;

                Ok(())
            }
            ExecuteSubcommand::On(relation, next) => {
                write!(f, "on {} {}", relation, next)?;

                Ok(())
            }
            ExecuteSubcommand::Positioned(positioned, next) => {
                write!(f, "positioned {} {}", positioned, next)?;

                Ok(())
            }
            ExecuteSubcommand::Rotated(rotated, next) => {
                write!(f, "rotated {} {}", rotated, next)?;

                Ok(())
            }
            ExecuteSubcommand::Summon(entity_id, next) => {
                write!(f, "summon {} {}", entity_id, next)?;

                Ok(())
            }
            ExecuteSubcommand::If(is_inverted, subcommand) => {
                let keyword = if *is_inverted { "unless" } else { "if" };

                write!(f, "{} {}", keyword, subcommand)
            }
            ExecuteSubcommand::Store(store_type, subcommand) => {
                write!(f, "store {} {}", store_type, subcommand)
            }
            ExecuteSubcommand::Run(command) => {
                write!(f, "run {}", command)
            }
        }
    }
}
