use std::fmt::{self, Display, Formatter};

use minecraft_command_types_procedural_macros::HasMacro;

use crate::{
    command::{
        Command,
        enums::score_operation_operator::ScoreOperationOperator,
        scoreboard::{PlayersScoreboardCommand, ScoreboardCommand},
    },
    entity_selector::EntitySelector,
    macroable::RegularMacroableExt,
    nbt_path::SNBTCompound,
    snbt::{SNBT, SNBTString},
};

pub type ScoreValue = i32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub struct PlayerScore {
    pub selector: EntitySelector,
    pub objective: String,
}

impl Display for PlayerScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.selector, self.objective)
    }
}

impl PlayerScore {
    #[must_use]
    pub const fn new(selector: EntitySelector, objective: String) -> Self {
        Self {
            selector,
            objective,
        }
    }

    #[must_use]
    pub fn to_text_component(self) -> SNBT {
        let mut score_compound = SNBTCompound::new();

        score_compound.insert(
            SNBTString(false, "name".to_owned()),
            SNBT::String(SNBTString(false, format!("{}", self.selector))).regular_macroable(),
        );

        score_compound.insert(
            SNBTString(false, "objective".to_owned()),
            SNBT::String(SNBTString(false, self.objective)).regular_macroable(),
        );

        let score_compound = SNBT::Compound(score_compound);

        let mut compound = SNBTCompound::new();

        compound.insert(
            SNBTString(false, "score".to_owned()),
            score_compound.regular_macroable(),
        );

        SNBT::Compound(compound)
    }

    #[inline]
    #[must_use]
    pub const fn get(self) -> Command {
        Command::Scoreboard(ScoreboardCommand::Players(PlayersScoreboardCommand::Get(
            self,
        )))
    }

    #[inline]
    #[must_use]
    pub const fn set_value(self, value: ScoreValue) -> Command {
        Command::Scoreboard(ScoreboardCommand::Players(PlayersScoreboardCommand::Set(
            self, value,
        )))
    }

    #[inline]
    #[must_use]
    pub const fn add_value(self, amount: ScoreValue) -> Command {
        Command::Scoreboard(ScoreboardCommand::Players(PlayersScoreboardCommand::Add(
            self, amount,
        )))
    }

    #[inline]
    #[must_use]
    pub const fn remove(self, amount: ScoreValue) -> Command {
        Command::Scoreboard(ScoreboardCommand::Players(
            PlayersScoreboardCommand::Remove(self, amount),
        ))
    }

    #[inline]
    #[must_use]
    pub const fn operation(
        self,
        operator: ScoreOperationOperator,
        other: Self,
    ) -> ScoreboardCommand {
        ScoreboardCommand::Players(PlayersScoreboardCommand::Operation(self, operator, other))
    }

    #[inline]
    #[must_use]
    pub const fn set(self, other: Self) -> ScoreboardCommand {
        self.operation(ScoreOperationOperator::Set, other)
    }

    #[inline]
    #[must_use]
    pub const fn add(self, other: Self) -> ScoreboardCommand {
        self.operation(ScoreOperationOperator::Add, other)
    }

    #[inline]
    #[must_use]
    pub const fn subtract(self, other: Self) -> ScoreboardCommand {
        self.operation(ScoreOperationOperator::Subtract, other)
    }

    #[inline]
    #[must_use]
    pub const fn multiply(self, other: Self) -> ScoreboardCommand {
        self.operation(ScoreOperationOperator::Multiply, other)
    }

    #[inline]
    #[must_use]
    pub const fn divide(self, other: Self) -> ScoreboardCommand {
        self.operation(ScoreOperationOperator::Divide, other)
    }

    #[inline]
    #[must_use]
    pub const fn modulo(self, other: Self) -> ScoreboardCommand {
        self.operation(ScoreOperationOperator::Remainder, other)
    }
}
