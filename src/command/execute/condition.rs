use crate::{
    block::BlockState,
    column_position::ColumnPosition,
    command::{
        Command,
        data::DataTarget,
        enums::if_blocks_mode::ConditionBlocksMode,
        execute::{ExecuteSubcommand, score_comparison::ScoreComparison},
        item_source::ItemSource,
    },
    coordinate::Coordinates,
    entity_selector::EntitySelector,
    item::ItemPredicate,
    nbt_path::NbtPath,
    option_write_chain,
    player_score::PlayerScore,
    range::FloatRange,
    resource_location::ResourceLocation,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecuteConditionSubcommand {
    Biome(
        Coordinates,
        ResourceLocation,
        Option<Box<ExecuteSubcommand>>,
    ),
    Block(Coordinates, BlockState, Option<Box<ExecuteSubcommand>>),
    Blocks {
        start: Coordinates,
        end: Coordinates,
        destination: Coordinates,
        mode: ConditionBlocksMode,
        next: Option<Box<ExecuteSubcommand>>,
    },
    Data(DataTarget, NbtPath, Option<Box<ExecuteSubcommand>>),
    Dimension(ResourceLocation, Option<Box<ExecuteSubcommand>>),
    Entity(EntitySelector, Option<Box<ExecuteSubcommand>>),
    Function(ResourceLocation, Box<ExecuteSubcommand>),
    Items {
        source: ItemSource,
        slots: String,
        predicate: ItemPredicate,
        next: Option<Box<ExecuteSubcommand>>,
    },
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
            Self::Blocks {
                start,
                end,
                destination,
                mode,
                next,
            } => {
                write!(f, "blocks {} {} {} {}", start, end, destination, mode)?;

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
            Self::Items {
                source,
                slots: slot,
                predicate,
                next,
            } => {
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
            Self::Blocks {
                start,
                end,
                destination,
                mode,
                next: inner_next,
            } => Self::Blocks {
                start,
                end,
                destination,
                mode,
                next: Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            },
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
            Self::Items {
                source,
                slots,
                predicate,
                next: inner_next,
            } => Self::Items {
                source,
                slots,
                predicate,
                next: Some(Box::new(match inner_next {
                    Some(inner_next) => inner_next.then(next),
                    None => next,
                })),
            },
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
            Self::Blocks { next, .. } => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Data(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Dimension(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Entity(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Function(..) => true,
            Self::Items { next, .. } => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Loaded(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Predicate(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Score(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
            Self::Stopwatch(.., next) => next.as_ref().is_some_and(|next| next.has_side_effects()),
        }
    }
}
