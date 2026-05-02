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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum Facing {
    Position(Coordinates),
    Entity(EntitySelector, EntityAnchor),
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::Entity(selector, anchor) => write!(f, "entity {} {}", selector, anchor),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum Positioned {
    Position(Coordinates),
    As(EntitySelector),
    Over(Heightmap),
}

impl Display for Positioned {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::As(selector) => write!(f, "as {}", selector),
            Self::Over(heightmap) => write!(f, "over {}", heightmap),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum Rotated {
    Rotation(Rotation),
    As(EntitySelector),
}

impl Display for Rotated {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rotation(rotation) => rotation.fmt(f),
            Self::As(selector) => write!(f, "as {}", selector),
        }
    }
}

#[derive(Display, Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ScoreComparison {
    Range(IntegerRange),
    Score(ScoreComparisonOperator, PlayerScore),
}

impl Display for ScoreComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Range(range) => write!(f, "matches {}", range),
            Self::Score(operator, right) => {
                write!(f, "{} {}", operator, right)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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
    Stopwatch(ResourceLocation, FloatRange, Option<Box<ExecuteSubcommand>>),
}

impl Display for ExecuteIfSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Biome(coords, id, next) => {
                write!(f, "biome {} {}", coords, id)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Block(coords, predicate, next) => {
                write!(f, "block {} {}", coords, predicate)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Blocks(start, end, dest, mode, next) => {
                write!(f, "blocks {} {} {} {}", start, end, dest, mode)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Data(target, path, next) => {
                write!(f, "data {} {}", target, path)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Dimension(id, next) => {
                write!(f, "dimension {}", id)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Entity(selector, next) => {
                write!(f, "entity {}", selector)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Function(id, next) => {
                write!(f, "function {} {}", id, next)
            }
            Self::Items(source, slot, predicate, next) => {
                write!(f, "items {} {} {}", source, slot, predicate)?;

                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Loaded(coords, next) => {
                write!(f, "loaded {}", coords)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Predicate(id, next) => {
                write!(f, "predicate {}", id)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Score(score, comparison, next) => {
                write!(f, "score {} {}", score, comparison)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
            Self::Stopwatch(location, range, next) => {
                write!(f, "stopwatch {} {}", location, range)?;
                if let Some(next_sub) = next {
                    write!(f, " {}", next_sub)?;
                }

                Ok(())
            }
        }
    }
}

impl ExecuteIfSubcommand {
    #[must_use]
    pub fn then(self, next: ExecuteSubcommand) -> Self {
        match self {
            Self::Biome(coordinates, resource_location, inner_next) => Self::Biome(
                coordinates,
                resource_location,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Block(coordinates, block_state, inner_next) => Self::Block(
                coordinates,
                block_state,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Blocks(coordinates, coordinates1, coordinates2, if_blocks_mode, inner_next) => {
                Self::Blocks(
                    coordinates,
                    coordinates1,
                    coordinates2,
                    if_blocks_mode,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            Self::Data(data_target, nbt_path, inner_next) => Self::Data(
                data_target,
                nbt_path,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Dimension(resource_location, inner_next) => Self::Dimension(
                resource_location,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Entity(entity_selector, inner_next) => Self::Entity(
                entity_selector,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Function(resource_location, inner_next) => {
                Self::Function(resource_location, Box::new(inner_next.then(next)))
            }
            Self::Items(item_source, slot, item_predicate, inner_next) => Self::Items(
                item_source,
                slot,
                item_predicate,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Loaded(column_position, inner_next) => Self::Loaded(
                column_position,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Predicate(resource_location, inner_next) => Self::Predicate(
                resource_location,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Score(player_score, score_comparison, inner_next) => Self::Score(
                player_score,
                score_comparison,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            Self::Stopwatch(resource_location, float_range, inner_next) => Self::Stopwatch(
                resource_location,
                float_range,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ExecuteStoreSubcommand {
    Data(
        DataTarget,
        NbtPath,
        NumericSNBTType,
        NotNan<f32>,
        Box<ExecuteSubcommand>,
    ),
    Bossbar(ResourceLocation, BossbarStoreType, Box<ExecuteSubcommand>),
    Score(PlayerScore, Box<ExecuteSubcommand>),
}

impl Display for ExecuteStoreSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data(target, path, num_type, scale, next) => {
                write!(f, "{} {} {} {} {}", target, path, num_type, scale, next)?;

                Ok(())
            }
            Self::Bossbar(id, store_type, next) => {
                write!(f, "bossbar {} {} {}", id, store_type, next)?;

                Ok(())
            }
            Self::Score(score, next) => {
                write!(f, "score {} {}", score, next)?;

                Ok(())
            }
        }
    }
}

impl ExecuteStoreSubcommand {
    #[must_use]
    pub fn then(self, next: ExecuteSubcommand) -> Self {
        match self {
            Self::Data(target, path, num_type, scale, inner_next) => Self::Data(
                target,
                path,
                num_type,
                scale,
                Box::new(inner_next.then(next)),
            ),
            Self::Bossbar(id, store_type, inner_next) => {
                Self::Bossbar(id, store_type, Box::new(inner_next.then(next)))
            }
            Self::Score(score, inner_next) => Self::Score(score, Box::new(inner_next.then(next))),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ExecuteSubcommand {
    Align(BTreeSet<Axis>, Box<Self>),
    Anchored(EntityAnchor, Box<Self>),
    As(EntitySelector, Box<Self>),
    At(EntitySelector, Box<Self>),
    Facing(Facing, Box<Self>),
    In(ResourceLocation, Box<Self>),
    On(Relation, Box<Self>),
    Positioned(Positioned, Box<Self>),
    Rotated(Rotated, Box<Self>),
    Summon(ResourceLocation, Box<Self>),
    If(bool, ExecuteIfSubcommand),
    Store(StoreType, ExecuteStoreSubcommand),
    Run(Box<Command>),
}

impl Display for ExecuteSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Align(axes, next) => {
                let axes_str: String = axes.iter().map(ToString::to_string).collect();
                write!(f, "align {} {}", axes_str, next)?;

                Ok(())
            }
            Self::Anchored(anchor, next) => {
                write!(f, "anchored {} {}", anchor, next)?;

                Ok(())
            }
            Self::As(selector, next) => {
                write!(f, "as {} {}", selector, next)?;

                Ok(())
            }
            Self::At(selector, next) => {
                write!(f, "at {} {}", selector, next)?;

                Ok(())
            }
            Self::Facing(facing, next) => {
                write!(f, "facing {} {}", facing, next)?;

                Ok(())
            }
            Self::In(dimension, next) => {
                write!(f, "in {} {}", dimension, next)?;

                Ok(())
            }
            Self::On(relation, next) => {
                write!(f, "on {} {}", relation, next)?;

                Ok(())
            }
            Self::Positioned(positioned, next) => {
                write!(f, "positioned {} {}", positioned, next)?;

                Ok(())
            }
            Self::Rotated(rotated, next) => {
                write!(f, "rotated {} {}", rotated, next)?;

                Ok(())
            }
            Self::Summon(entity_id, next) => {
                write!(f, "summon {} {}", entity_id, next)?;

                Ok(())
            }
            Self::If(is_inverted, subcommand) => {
                let keyword = if *is_inverted { "unless" } else { "if" };

                write!(f, "{} {}", keyword, subcommand)
            }
            Self::Store(store_type, subcommand) => {
                write!(f, "store {} {}", store_type, subcommand)
            }
            Self::Run(command) => {
                write!(f, "run {}", command)
            }
        }
    }
}

impl ExecuteSubcommand {
    #[inline]
    #[must_use]
    pub fn run_return_value_0() -> Self {
        Self::Run(Box::new(Command::RETURN_VALUE_0))
    }

    #[inline]
    #[must_use]
    pub fn run_return_value_1() -> Self {
        Self::Run(Box::new(Command::RETURN_VALUE_1))
    }

    #[inline]
    #[must_use]
    pub fn run_return_fail() -> Self {
        Self::Run(Box::new(Command::RETURN_FAIL))
    }

    #[must_use]
    pub fn then(self, next: Self) -> Self {
        match self {
            Self::Align(axes, inner_next) => Self::Align(axes, Box::new(inner_next.then(next))),
            Self::Anchored(anchor, inner_next) => {
                Self::Anchored(anchor, Box::new(inner_next.then(next)))
            }
            Self::As(selector, inner_next) => Self::As(selector, Box::new(inner_next.then(next))),
            Self::At(selector, inner_next) => Self::At(selector, Box::new(inner_next.then(next))),
            Self::Facing(facing, inner_next) => {
                Self::Facing(facing, Box::new(inner_next.then(next)))
            }
            Self::In(resource_location, inner_next) => {
                Self::In(resource_location, Box::new(inner_next.then(next)))
            }
            Self::On(relation, inner_next) => Self::On(relation, Box::new(inner_next.then(next))),
            Self::Positioned(positioned, inner_next) => {
                Self::Positioned(positioned, Box::new(inner_next.then(next)))
            }
            Self::Rotated(rotated, inner_next) => {
                Self::Rotated(rotated, Box::new(inner_next.then(next)))
            }
            Self::Summon(resource_location, inner_next) => {
                Self::Summon(resource_location, Box::new(inner_next.then(next)))
            }
            Self::If(inverted, subcommand) => Self::If(inverted, subcommand.then(next)),
            Self::Store(store_type, subcommand) => Self::Store(store_type, subcommand.then(next)),
            Self::Run(_) => next.then(self),
        }
    }
}
