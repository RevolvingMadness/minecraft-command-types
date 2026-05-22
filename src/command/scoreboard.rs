use crate::command::enums::score_operation_operator::ScoreOperationOperator;
use crate::command::enums::scoreboard_render_type::ScoreboardRenderType;
use crate::command::{Command, ScoreValue};
use crate::entity_selector::EntitySelector;
use crate::option_write_chain;
use crate::snbt::SNBT;
use crate::{command::PlayerScore, macroable::Macroable};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ScoreboardNumberFormat {
    Blank,
    Fixed(Macroable<SNBT>),
    Styled(Macroable<SNBT>),
}

impl Display for ScoreboardNumberFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => f.write_str("blank"),
            Self::Fixed(snbt) => write!(f, "fixed {}", snbt),
            Self::Styled(style) => write!(f, "styled {}", style),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ScoreboardModification {
    DisplayAutoUpdate(bool),
    DisplayName(Macroable<SNBT>),
    NumberFormat(Option<ScoreboardNumberFormat>),
    RenderType(ScoreboardRenderType),
}

impl Display for ScoreboardModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DisplayAutoUpdate(value) => {
                write!(f, "displayautoupdate {}", value)
            }
            Self::DisplayName(display_name) => {
                write!(f, "displayname {}", display_name)
            }
            Self::NumberFormat(number_format) => {
                f.write_str("numberformat")?;

                option_write_chain!(f, number_format);

                Ok(())
            }
            Self::RenderType(render_type) => {
                write!(f, "rendertype {}", render_type)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ObjectivesScoreboardCommand {
    List,
    Add(String, String, Option<Macroable<SNBT>>),
    Remove(String),
    SetDisplay(String, Option<String>),
    Modify(String, ScoreboardModification),
}

impl Display for ObjectivesScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List => f.write_str("list"),
            Self::Add(name, criterion, display_name) => {
                write!(f, "add {} {}", name, criterion)?;

                option_write_chain!(f, display_name);

                Ok(())
            }
            Self::Remove(name) => {
                write!(f, "remove {}", name)
            }
            Self::SetDisplay(slot, name) => {
                write!(f, "setdisplay {}", slot)?;

                option_write_chain!(f, name);

                Ok(())
            }
            Self::Modify(name, modification) => {
                write!(f, "modify {} {}", name, modification)
            }
        }
    }
}

impl From<ObjectivesScoreboardCommand> for ScoreboardCommand {
    fn from(value: ObjectivesScoreboardCommand) -> Self {
        Self::Objectives(value)
    }
}

impl From<ObjectivesScoreboardCommand> for Command {
    fn from(value: ObjectivesScoreboardCommand) -> Self {
        Self::Scoreboard(value.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum PlayersDisplayScoreboardCommand {
    Name(PlayerScore, Option<Macroable<SNBT>>),
    NumberFormat(PlayerScore, Option<ScoreboardNumberFormat>),
}

impl Display for PlayersDisplayScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(score, text) => {
                write!(f, "name {}", score)?;

                option_write_chain!(f, text);

                Ok(())
            }
            Self::NumberFormat(score, number_format) => {
                write!(f, "numberformat {}", score)?;

                option_write_chain!(f, number_format);

                Ok(())
            }
        }
    }
}

impl From<PlayersDisplayScoreboardCommand> for PlayersScoreboardCommand {
    fn from(value: PlayersDisplayScoreboardCommand) -> Self {
        Self::Display(value)
    }
}

impl From<PlayersDisplayScoreboardCommand> for ScoreboardCommand {
    fn from(value: PlayersDisplayScoreboardCommand) -> Self {
        Self::Players(value.into())
    }
}

impl From<PlayersDisplayScoreboardCommand> for Command {
    fn from(value: PlayersDisplayScoreboardCommand) -> Self {
        Self::Scoreboard(value.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum PlayersScoreboardCommand {
    List(Option<EntitySelector>),
    Get(PlayerScore),
    Set(PlayerScore, ScoreValue),
    Add(PlayerScore, ScoreValue),
    Remove(PlayerScore, ScoreValue),
    Reset(EntitySelector, Option<String>),
    Enable(PlayerScore),
    Operation(PlayerScore, ScoreOperationOperator, PlayerScore),
    Display(PlayersDisplayScoreboardCommand),
}

impl Display for PlayersScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(selector) => {
                f.write_str("list")?;

                option_write_chain!(f, selector);

                Ok(())
            }
            Self::Get(score) => {
                write!(f, "get {}", score)
            }
            Self::Set(score, value) => {
                write!(f, "set {} {}", score, value)
            }
            Self::Add(score, value) => {
                if *value >= 0 {
                    write!(f, "add {} {}", score, value)
                } else {
                    write!(f, "remove {} {}", score, -value)
                }
            }
            Self::Remove(score, value) => {
                if *value >= 0 {
                    write!(f, "remove {} {}", score, value)
                } else {
                    write!(f, "add {} {}", score, -value)
                }
            }
            Self::Reset(selector, objective) => {
                write!(f, "reset {}", selector)?;

                option_write_chain!(f, objective);

                Ok(())
            }
            Self::Enable(score) => {
                write!(f, "enable {}", score)
            }
            Self::Operation(left, operator, right) => {
                write!(f, "operation {} {} {}", left, operator, right)
            }
            Self::Display(command) => {
                write!(f, "display {}", command)
            }
        }
    }
}

impl From<PlayersScoreboardCommand> for ScoreboardCommand {
    fn from(value: PlayersScoreboardCommand) -> Self {
        Self::Players(value)
    }
}

impl From<PlayersScoreboardCommand> for Command {
    fn from(value: PlayersScoreboardCommand) -> Self {
        Self::Scoreboard(value.into())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ScoreboardCommand {
    Objectives(ObjectivesScoreboardCommand),
    Players(PlayersScoreboardCommand),
}

impl Display for ScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Objectives(command) => write!(f, "objectives {}", command),
            Self::Players(command) => write!(f, "players {}", command),
        }
    }
}

impl From<ScoreboardCommand> for Command {
    fn from(value: ScoreboardCommand) -> Self {
        Self::Scoreboard(value)
    }
}
