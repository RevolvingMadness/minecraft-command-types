use crate::{
    command::{
        Command,
        data::DataTarget,
        enums::{
            axis::Axis, entity_anchor::EntityAnchor, numeric_snbt_type::NumericSnbtType,
            relation::Relation, store_type::StoreType,
        },
        execute::{
            condition::ExecuteConditionSubcommand,
            facing::Facing,
            positioned::Positioned,
            rotated::Rotated,
            score_comparison::{ScoreComparison, ScoreComparisonOperator},
            store::ExecuteStoreSubcommand,
        },
    },
    entity_selector::EntitySelector,
    nbt_path::NbtPath,
    player_score::PlayerScore,
    range::IntegerRange,
    resource_location::ResourceLocation,
    types::Float,
};
use itertools::Itertools;
use std::{
    collections::BTreeSet,
    fmt::{self, Display, Formatter},
};

pub mod condition;
pub mod facing;
pub mod positioned;
pub mod rotated;
pub mod score_comparison;
pub mod store;

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
                write!(f, "align {} {}", axes.iter().format(""), next)?;

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
