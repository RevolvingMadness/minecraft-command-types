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
            Facing::Position(coords) => coords.fmt(f),
            Facing::Entity(selector, anchor) => write!(f, "entity {} {}", selector, anchor),
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
            Positioned::Position(coords) => coords.fmt(f),
            Positioned::As(selector) => write!(f, "as {}", selector),
            Positioned::Over(heightmap) => write!(f, "over {}", heightmap),
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
            Rotated::Rotation(rotation) => rotation.fmt(f),
            Rotated::As(selector) => write!(f, "as {}", selector),
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
            ScoreComparison::Range(range) => write!(f, "matches {}", range),
            ScoreComparison::Score(operator, right) => {
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
            ExecuteIfSubcommand::Stopwatch(location, range, next) => {
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
    pub fn then(self, next: ExecuteSubcommand) -> ExecuteIfSubcommand {
        match self {
            ExecuteIfSubcommand::Biome(coordinates, resource_location, inner_next) => {
                ExecuteIfSubcommand::Biome(
                    coordinates,
                    resource_location,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Block(coordinates, block_state, inner_next) => {
                ExecuteIfSubcommand::Block(
                    coordinates,
                    block_state,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Blocks(
                coordinates,
                coordinates1,
                coordinates2,
                if_blocks_mode,
                inner_next,
            ) => ExecuteIfSubcommand::Blocks(
                coordinates,
                coordinates1,
                coordinates2,
                if_blocks_mode,
                Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            ),
            ExecuteIfSubcommand::Data(data_target, nbt_path, inner_next) => {
                ExecuteIfSubcommand::Data(
                    data_target,
                    nbt_path,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Dimension(resource_location, inner_next) => {
                ExecuteIfSubcommand::Dimension(
                    resource_location,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Entity(entity_selector, inner_next) => {
                ExecuteIfSubcommand::Entity(
                    entity_selector,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Function(resource_location, inner_next) => {
                ExecuteIfSubcommand::Function(resource_location, Box::new(inner_next.then(next)))
            }
            ExecuteIfSubcommand::Items(item_source, slot, item_predicate, inner_next) => {
                ExecuteIfSubcommand::Items(
                    item_source,
                    slot,
                    item_predicate,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Loaded(column_position, inner_next) => {
                ExecuteIfSubcommand::Loaded(
                    column_position,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Predicate(resource_location, inner_next) => {
                ExecuteIfSubcommand::Predicate(
                    resource_location,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Score(player_score, score_comparison, inner_next) => {
                ExecuteIfSubcommand::Score(
                    player_score,
                    score_comparison,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
            ExecuteIfSubcommand::Stopwatch(resource_location, float_range, inner_next) => {
                ExecuteIfSubcommand::Stopwatch(
                    resource_location,
                    float_range,
                    Some(Box::new(match inner_next {
                        Some(inner_next) => inner_next.then(next),
                        None => next,
                    })),
                )
            }
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
            ExecuteStoreSubcommand::Data(target, path, num_type, scale, next) => {
                write!(f, "{} {} {} {} {}", target, path, num_type, scale, next)?;

                Ok(())
            }
            ExecuteStoreSubcommand::Bossbar(id, store_type, next) => {
                write!(f, "bossbar {} {} {}", id, store_type, next)?;

                Ok(())
            }
            ExecuteStoreSubcommand::Score(score, next) => {
                write!(f, "score {} {}", score, next)?;

                Ok(())
            }
        }
    }
}

impl ExecuteStoreSubcommand {
    pub fn then(self, next: ExecuteSubcommand) -> ExecuteStoreSubcommand {
        match self {
            ExecuteStoreSubcommand::Data(target, path, num_type, scale, inner_next) => {
                ExecuteStoreSubcommand::Data(
                    target,
                    path,
                    num_type,
                    scale,
                    Box::new(inner_next.then(next)),
                )
            }
            ExecuteStoreSubcommand::Bossbar(id, store_type, inner_next) => {
                ExecuteStoreSubcommand::Bossbar(id, store_type, Box::new(inner_next.then(next)))
            }
            ExecuteStoreSubcommand::Score(score, inner_next) => {
                ExecuteStoreSubcommand::Score(score, Box::new(inner_next.then(next)))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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

impl ExecuteSubcommand {
    pub fn then(self, next: ExecuteSubcommand) -> ExecuteSubcommand {
        match self {
            ExecuteSubcommand::Align(axes, inner_next) => {
                ExecuteSubcommand::Align(axes, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::Anchored(anchor, inner_next) => {
                ExecuteSubcommand::Anchored(anchor, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::As(selector, inner_next) => {
                ExecuteSubcommand::As(selector, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::At(selector, inner_next) => {
                ExecuteSubcommand::At(selector, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::Facing(facing, inner_next) => {
                ExecuteSubcommand::Facing(facing, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::In(resource_location, inner_next) => {
                ExecuteSubcommand::In(resource_location, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::On(relation, inner_next) => {
                ExecuteSubcommand::On(relation, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::Positioned(positioned, inner_next) => {
                ExecuteSubcommand::Positioned(positioned, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::Rotated(rotated, inner_next) => {
                ExecuteSubcommand::Rotated(rotated, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::Summon(resource_location, inner_next) => {
                ExecuteSubcommand::Summon(resource_location, Box::new(inner_next.then(next)))
            }
            ExecuteSubcommand::If(inverted, subcommand) => {
                ExecuteSubcommand::If(inverted, subcommand.then(next))
            }
            ExecuteSubcommand::Store(store_type, subcommand) => {
                ExecuteSubcommand::Store(store_type, subcommand.then(next))
            }
            ExecuteSubcommand::Run(_) => next.then(self),
        }
    }
}
