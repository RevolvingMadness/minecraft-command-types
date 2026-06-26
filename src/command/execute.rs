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
use crate::option_write_chain;
use crate::range::{FloatRange, IntegerRange};
use crate::resource_location::ResourceLocation;
use crate::rotation::Rotation;
use minecraft_command_types_procedural_macros::HasMacro;
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

#[derive(Display, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
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

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Block(coords, predicate, next) => {
                write!(f, "block {} {}", coords, predicate)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Blocks(start, end, dest, mode, next) => {
                write!(f, "blocks {} {} {} {}", start, end, dest, mode)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Data(target, path, next) => {
                write!(f, "data {} {}", target, path)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Dimension(id, next) => {
                write!(f, "dimension {}", id)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Entity(selector, next) => {
                write!(f, "entity {}", selector)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Function(id, next) => {
                write!(f, "function {} {}", id, next)
            }
            Self::Items(source, slot, predicate, next) => {
                write!(f, "items {} {} {}", source, slot, predicate)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Loaded(coords, next) => {
                write!(f, "loaded {}", coords)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Predicate(id, next) => {
                write!(f, "predicate {}", id)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Score(score, comparison, next) => {
                write!(f, "score {} {}", score, comparison)?;

                option_write_chain!(f, next);

                Ok(())
            }
            Self::Stopwatch(location, range, next) => {
                write!(f, "stopwatch {} {}", location, range)?;

                option_write_chain!(f, next);

                Ok(())
            }
        }
    }
}

impl From<ExecuteIfSubcommand> for ExecuteSubcommand {
    fn from(value: ExecuteIfSubcommand) -> Self {
        Self::If(false, value)
    }
}

impl From<ExecuteIfSubcommand> for Command {
    fn from(value: ExecuteIfSubcommand) -> Self {
        Self::Execute(value.into())
    }
}

impl ExecuteIfSubcommand {
    #[inline]
    #[must_use]
    pub const fn into_subcommand(self, inverted: bool) -> ExecuteSubcommand {
        ExecuteSubcommand::If(inverted, self)
    }

    #[inline]
    #[must_use]
    pub const fn if_(self) -> ExecuteSubcommand {
        self.into_subcommand(false)
    }

    #[inline]
    #[must_use]
    pub const fn unless(self) -> ExecuteSubcommand {
        self.into_subcommand(true)
    }

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

    #[must_use]
    pub fn has_side_effects(&self) -> bool {
        match self {
            Self::Biome(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Block(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Blocks(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Data(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Dimension(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Entity(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Function(..) => true,
            Self::Items(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Loaded(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Predicate(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Score(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Stopwatch(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
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

impl From<ExecuteSubcommand> for Command {
    fn from(value: ExecuteSubcommand) -> Self {
        Self::Execute(value)
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
    pub fn then<N: Into<Self>>(self, next: N) -> Self {
        let next = next.into();

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
            Self::Run(..) => next.then(self),
        }
    }

    #[inline]
    #[must_use]
    pub fn store_score(self, store_type: StoreType, score: PlayerScore) -> Self {
        Self::Store(
            store_type,
            ExecuteStoreSubcommand::Score(score, Box::new(self)),
        )
    }

    #[inline]
    #[must_use]
    pub fn store_result_score(self, score: PlayerScore) -> Self {
        self.store_score(StoreType::Result, score)
    }

    #[inline]
    #[must_use]
    pub fn store_success_score(self, score: PlayerScore) -> Self {
        self.store_score(StoreType::Success, score)
    }

    #[inline]
    #[must_use]
    pub fn store_data(
        self,
        store_type: StoreType,
        target: DataTarget,
        path: NbtPath,
        nbt_type: NumericSNBTType,
        scale: NotNan<f32>,
    ) -> Self {
        Self::Store(
            store_type,
            ExecuteStoreSubcommand::Data(target, path, nbt_type, scale, Box::new(self)),
        )
    }

    #[inline]
    #[must_use]
    pub fn store_result_data(
        self,
        target: DataTarget,
        path: NbtPath,
        nbt_type: NumericSNBTType,
        scale: NotNan<f32>,
    ) -> Self {
        self.store_data(StoreType::Result, target, path, nbt_type, scale)
    }

    #[inline]
    #[must_use]
    pub fn store_success_data(
        self,
        target: DataTarget,
        path: NbtPath,
        nbt_type: NumericSNBTType,
        scale: NotNan<f32>,
    ) -> Self {
        self.store_data(StoreType::Success, target, path, nbt_type, scale)
    }

    #[inline]
    #[must_use]
    pub fn condition_data(self, inverted: bool, target: DataTarget, path: NbtPath) -> Self {
        Self::If(
            inverted,
            ExecuteIfSubcommand::Data(target, path, Some(Box::new(self))),
        )
    }

    #[inline]
    #[must_use]
    pub fn if_data(self, target: DataTarget, path: NbtPath) -> Self {
        self.condition_data(false, target, path)
    }

    #[inline]
    #[must_use]
    pub fn unless_data(self, target: DataTarget, path: NbtPath) -> Self {
        self.condition_data(true, target, path)
    }

    #[inline]
    #[must_use]
    pub fn condition_function(self, inverted: bool, resource_location: ResourceLocation) -> Self {
        Self::If(
            inverted,
            ExecuteIfSubcommand::Function(resource_location, Box::new(self)),
        )
    }

    #[inline]
    #[must_use]
    pub fn if_function(self, resource_location: ResourceLocation) -> Self {
        self.condition_function(false, resource_location)
    }

    #[inline]
    #[must_use]
    pub fn unless_function(self, resource_location: ResourceLocation) -> Self {
        self.condition_function(true, resource_location)
    }

    #[inline]
    #[must_use]
    pub fn conditionally(self, inverted: bool, subcommand: ExecuteIfSubcommand) -> Self {
        Self::If(inverted, subcommand.then(self))
    }

    #[inline]
    #[must_use]
    pub fn if_(self, subcommand: ExecuteIfSubcommand) -> Self {
        self.conditionally(false, subcommand)
    }

    #[inline]
    #[must_use]
    pub fn unless(self, subcommand: ExecuteIfSubcommand) -> Self {
        self.conditionally(true, subcommand)
    }

    #[inline]
    #[must_use]
    pub fn condition_score_range(
        self,
        inverted: bool,
        score: PlayerScore,
        min: Option<i32>,
        max: Option<i32>,
    ) -> Self {
        Self::If(
            inverted,
            ExecuteIfSubcommand::Score(
                score,
                ScoreComparison::Range(IntegerRange { min, max }),
                Some(Box::new(self)),
            ),
        )
    }

    #[inline]
    #[must_use]
    pub fn if_score_range(self, score: PlayerScore, min: Option<i32>, max: Option<i32>) -> Self {
        self.condition_score_range(false, score, min, max)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_range(
        self,
        score: PlayerScore,
        min: Option<i32>,
        max: Option<i32>,
    ) -> Self {
        self.condition_score_range(true, score, min, max)
    }

    #[inline]
    #[must_use]
    pub fn condition_operator(
        self,
        inverted: bool,
        left: PlayerScore,
        operator: ScoreComparisonOperator,
        right: PlayerScore,
    ) -> Self {
        Self::If(
            inverted,
            ExecuteIfSubcommand::Score(
                left,
                ScoreComparison::Score(operator, right),
                Some(Box::new(self)),
            ),
        )
    }

    #[inline]
    #[must_use]
    pub fn if_score_operator(
        self,
        left: PlayerScore,
        operator: ScoreComparisonOperator,
        right: PlayerScore,
    ) -> Self {
        self.condition_operator(false, left, operator, right)
    }

    #[inline]
    #[must_use]
    pub fn if_score_less_than(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.if_score_operator(left, ScoreComparisonOperator::LessThan, right)
    }

    #[inline]
    #[must_use]
    pub fn if_score_less_than_or_equal_to(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.if_score_operator(left, ScoreComparisonOperator::LessThanOrEqualTo, right)
    }

    #[inline]
    #[must_use]
    pub fn if_score_equal_to(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.if_score_operator(left, ScoreComparisonOperator::EqualTo, right)
    }

    #[inline]
    #[must_use]
    pub fn if_score_greater_than(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.if_score_operator(left, ScoreComparisonOperator::GreaterThan, right)
    }

    #[inline]
    #[must_use]
    pub fn if_score_greater_than_or_equal_to(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.if_score_operator(left, ScoreComparisonOperator::GreaterThanOrEqualTo, right)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_operator(
        self,
        left: PlayerScore,
        operator: ScoreComparisonOperator,
        right: PlayerScore,
    ) -> Self {
        self.condition_operator(true, left, operator, right)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_less_than(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.unless_score_operator(left, ScoreComparisonOperator::LessThan, right)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_less_than_or_equal_to(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.unless_score_operator(left, ScoreComparisonOperator::LessThanOrEqualTo, right)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_equal_to(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.unless_score_operator(left, ScoreComparisonOperator::EqualTo, right)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_greater_than(self, left: PlayerScore, right: PlayerScore) -> Self {
        self.unless_score_operator(left, ScoreComparisonOperator::GreaterThan, right)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_greater_than_or_equal_to(
        self,
        left: PlayerScore,
        right: PlayerScore,
    ) -> Self {
        self.unless_score_operator(left, ScoreComparisonOperator::GreaterThanOrEqualTo, right)
    }

    #[must_use]
    pub fn has_side_effects(&self) -> bool {
        match self {
            Self::Align(.., next) => next.has_side_effects(),
            Self::Anchored(.., next) => next.has_side_effects(),
            Self::As(.., next) => next.has_side_effects(),
            Self::At(.., next) => next.has_side_effects(),
            Self::Facing(.., next) => next.has_side_effects(),
            Self::In(.., next) => next.has_side_effects(),
            Self::On(.., next) => next.has_side_effects(),
            Self::Positioned(.., next) => next.has_side_effects(),
            Self::Rotated(.., next) => next.has_side_effects(),
            Self::Summon(..) => true,
            Self::If(.., subcommand) => subcommand.has_side_effects(),
            Self::Store(..) => true,
            Self::Run(command) => command.has_side_effects(),
        }
    }
}
