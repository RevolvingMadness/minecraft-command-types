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
pub enum ExecuteModifier {
    Align(BTreeSet<Axis>),
    Anchored(EntityAnchor),
    As(EntitySelector),
    At(EntitySelector),
    Facing(Facing),
    In(ResourceLocation),
    On(Relation),
    Positioned(Positioned),
    Rotated(Rotated),
    Summon(ResourceLocation),
}

impl Display for ExecuteModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Align(axes) => {
                write!(f, "align {}", axes.iter().format(""))?;

                Ok(())
            }
            Self::Anchored(anchor) => {
                write!(f, "anchored {}", anchor)?;

                Ok(())
            }
            Self::As(selector) => {
                write!(f, "as {}", selector)?;

                Ok(())
            }
            Self::At(selector) => {
                write!(f, "at {}", selector)?;

                Ok(())
            }
            Self::Facing(facing) => {
                write!(f, "facing {}", facing)?;

                Ok(())
            }
            Self::In(dimension) => {
                write!(f, "in {}", dimension)?;

                Ok(())
            }
            Self::On(relation) => {
                write!(f, "on {}", relation)?;

                Ok(())
            }
            Self::Positioned(positioned) => {
                write!(f, "positioned {}", positioned)?;

                Ok(())
            }
            Self::Rotated(rotated) => {
                write!(f, "rotated {}", rotated)?;

                Ok(())
            }
            Self::Summon(entity_id) => {
                write!(f, "summon {}", entity_id)?;

                Ok(())
            }
        }
    }
}

impl ExecuteModifier {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Align(..) => false,
            Self::Anchored(..) => false,
            Self::As(..) => false,
            Self::At(..) => false,
            Self::Facing(..) => false,
            Self::In(..) => false,
            Self::On(..) => false,
            Self::Positioned(..) => false,
            Self::Rotated(..) => false,
            Self::Summon(..) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecuteSubcommand {
    Modifier(ExecuteModifier, Box<Self>),
    If(bool, ExecuteConditionSubcommand),
    Store(StoreType, ExecuteStoreSubcommand),
    Run(Box<Command>),
}

impl Display for ExecuteSubcommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Modifier(modifier, next) => {
                write!(f, "{} {}", modifier, next)
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
    pub fn then<S: Into<Self>>(self, next: S) -> Option<Self> {
        let next = next.into();

        Some(match self {
            Self::Modifier(modifier, inner_next) => {
                Self::Modifier(modifier, Box::new(inner_next.then(next)?))
            }
            Self::If(inverted, subcommand) => Self::If(inverted, subcommand.then(next)?),
            Self::Store(store_type, subcommand) => Self::Store(store_type, subcommand.then(next)?),
            Self::Run(..) => return None,
        })
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
    pub fn conditionally(
        self,
        inverted: bool,
        subcommand: ExecuteConditionSubcommand,
    ) -> Option<Self> {
        Some(Self::If(inverted, subcommand.then(self)?))
    }

    #[inline]
    #[must_use]
    pub fn if_(self, subcommand: ExecuteConditionSubcommand) -> Option<Self> {
        self.conditionally(false, subcommand)
    }

    #[inline]
    #[must_use]
    pub fn unless(self, subcommand: ExecuteConditionSubcommand) -> Option<Self> {
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
            Self::Modifier(modifier, next) => {
                modifier.has_side_effects() || next.has_side_effects()
            }
            Self::If(.., subcommand) => subcommand.has_side_effects(),
            Self::Store(..) => true,
            Self::Run(command) => command.has_side_effects(),
        }
    }
}
