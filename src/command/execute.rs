use crate::{
    block::BlockState,
    column_position::ColumnPosition,
    command::{
        Command,
        data::DataTarget,
        enums::{
            axis::Axis, bossbar_store_type::BossbarStoreType, entity_anchor::EntityAnchor,
            heightmap::Heightmap, if_blocks_mode::IfBlocksMode, numeric_snbt_type::NumericSnbtType,
            relation::Relation, store_type::StoreType,
        },
        item_source::ItemSource,
    },
    coordinate::Coordinates,
    entity_selector::EntitySelector,
    item::ItemPredicate,
    nbt_path::NbtPath,
    option_write_chain,
    player_score::PlayerScore,
    range::{FloatRange, IntegerRange},
    resource_location::ResourceLocation,
    rotation::Rotation,
    types::Float,
};
use std::{
    collections::BTreeSet,
    fmt::{self, Display, Formatter},
};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Facing {
    Position(Coordinates),
    Entity(EntitySelector, EntityAnchor),
}

impl Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::Entity(selector, anchor) => write!(f, "entity {} {}", selector, anchor),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Positioned {
    Position(Coordinates),
    As(EntitySelector),
    Over(Heightmap),
}

impl Display for Positioned {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Position(coords) => coords.fmt(f),
            Self::As(selector) => write!(f, "as {}", selector),
            Self::Over(heightmap) => write!(f, "over {}", heightmap),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rotated {
    Rotation(Rotation),
    As(EntitySelector),
}

impl Display for Rotated {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rotation(rotation) => rotation.fmt(f),
            Self::As(selector) => write!(f, "as {}", selector),
        }
    }
}

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl ScoreComparisonOperator {
    #[must_use]
    pub const fn into_range(self, value: i32) -> IntegerRange {
        match self {
            Self::LessThan => IntegerRange::new_upper(value - 1),
            Self::LessThanOrEqualTo => IntegerRange::new_upper(value),
            Self::EqualTo => IntegerRange::new_single(value),
            Self::GreaterThan => IntegerRange::new_lower(value + 1),
            Self::GreaterThanOrEqualTo => IntegerRange::new_lower(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreComparison {
    Range(IntegerRange),
    Score(ScoreComparisonOperator, PlayerScore),
}

impl Display for ScoreComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range(range) => write!(f, "matches {}", range),
            Self::Score(operator, right) => {
                write!(f, "{} {}", operator, right)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecuteConditionSubcommand {
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

impl Display for ExecuteConditionSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl From<ExecuteConditionSubcommand> for ExecuteSubcommand {
    fn from(value: ExecuteConditionSubcommand) -> Self {
        Self::If(false, value)
    }
}

impl From<ExecuteConditionSubcommand> for Command {
    fn from(value: ExecuteConditionSubcommand) -> Self {
        Self::Execute(value.into())
    }
}

impl ExecuteConditionSubcommand {
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
    pub fn then<S: Into<ExecuteSubcommand>>(self, next: S) -> Self {
        let next = next.into();

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecuteStoreSubcommand {
    Data(
        DataTarget,
        NbtPath,
        NumericSnbtType,
        Float,
        Box<ExecuteSubcommand>,
    ),
    Bossbar(ResourceLocation, BossbarStoreType, Box<ExecuteSubcommand>),
    Score(PlayerScore, Box<ExecuteSubcommand>),
}

impl Display for ExecuteStoreSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    If(bool, ExecuteConditionSubcommand),
    Store(StoreType, ExecuteStoreSubcommand),
    Run(Box<Command>),
}

impl Display for ExecuteSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
                let keyword = if *is_inverted {
                    "unless"
                } else {
                    "if"
                };

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
    #[must_use]
    pub fn then<S: Into<Self>>(self, next: S) -> Self {
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
        nbt_type: NumericSnbtType,
        scale: Float,
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
        nbt_type: NumericSnbtType,
        scale: Float,
    ) -> Self {
        self.store_data(StoreType::Result, target, path, nbt_type, scale)
    }

    #[inline]
    #[must_use]
    pub fn store_success_data(
        self,
        target: DataTarget,
        path: NbtPath,
        nbt_type: NumericSnbtType,
        scale: Float,
    ) -> Self {
        self.store_data(StoreType::Success, target, path, nbt_type, scale)
    }

    #[inline]
    #[must_use]
    pub fn condition_data(self, inverted: bool, target: DataTarget, path: NbtPath) -> Self {
        Self::If(
            inverted,
            ExecuteConditionSubcommand::Data(target, path, Some(Box::new(self))),
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
            ExecuteConditionSubcommand::Function(resource_location, Box::new(self)),
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
    pub fn conditionally(self, inverted: bool, subcommand: ExecuteConditionSubcommand) -> Self {
        Self::If(inverted, subcommand.then(self))
    }

    #[inline]
    #[must_use]
    pub fn if_(self, subcommand: ExecuteConditionSubcommand) -> Self {
        self.conditionally(false, subcommand)
    }

    #[inline]
    #[must_use]
    pub fn unless(self, subcommand: ExecuteConditionSubcommand) -> Self {
        self.conditionally(true, subcommand)
    }

    #[inline]
    #[must_use]
    pub fn condition_score_range(
        self,
        inverted: bool,
        score: PlayerScore,
        range: IntegerRange,
    ) -> Self {
        Self::If(
            inverted,
            ExecuteConditionSubcommand::Score(
                score,
                ScoreComparison::Range(range),
                Some(Box::new(self)),
            ),
        )
    }

    #[inline]
    #[must_use]
    pub fn if_score_range(self, score: PlayerScore, range: IntegerRange) -> Self {
        self.condition_score_range(false, score, range)
    }

    #[inline]
    #[must_use]
    pub fn unless_score_range(self, score: PlayerScore, range: IntegerRange) -> Self {
        self.condition_score_range(true, score, range)
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
            ExecuteConditionSubcommand::Score(
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
