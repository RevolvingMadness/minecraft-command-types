use crate::command::enums::score_operation_operator::ScoreOperationOperator;
use crate::command::enums::scoreboard_criterion::ScoreboardCriterion;
use crate::command::enums::scoreboard_render_type::ScoreboardRenderType;
use crate::command::enums::team_color::TeamColor;
use crate::command::PlayerScore;
use crate::entity_selector::EntitySelector;
use crate::snbt::SNBT;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScoreboardNumberFormat {
    Blank,
    Fixed(SNBT),
    Styled(SNBT),
}

impl Display for ScoreboardNumberFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardNumberFormat::Blank => f.write_str("blank"),
            ScoreboardNumberFormat::Fixed(snbt) => write!(f, "fixed {}", snbt),
            ScoreboardNumberFormat::Styled(style) => write!(f, "styled {}", style),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScoreboardModification {
    DisplayAutoUpdate(bool),
    DisplayName(SNBT),
    NumberFormat(Option<ScoreboardNumberFormat>),
    RenderType(ScoreboardRenderType),
}

impl Display for ScoreboardModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardModification::DisplayAutoUpdate(value) => {
                write!(f, "displayautoupdate {}", value)
            }
            ScoreboardModification::DisplayName(display_name) => {
                write!(f, "displayname {}", display_name)
            }
            ScoreboardModification::NumberFormat(number_format) => {
                f.write_str("numberformat")?;

                if let Some(number_format) = number_format {
                    write!(f, " {}", number_format)?;
                }

                Ok(())
            }
            ScoreboardModification::RenderType(render_type) => {
                write!(f, "rendertype {}", render_type)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScoreboardDisplaySlot {
    List,
    Sidebar,
    SidebarTeam(TeamColor),
    BelowName,
}

impl Display for ScoreboardDisplaySlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardDisplaySlot::List => f.write_str("list"),
            ScoreboardDisplaySlot::Sidebar => f.write_str("sidebar"),
            ScoreboardDisplaySlot::SidebarTeam(color) => write!(f, "sidebar.team.{}", color),
            ScoreboardDisplaySlot::BelowName => f.write_str("below_name"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ObjectivesScoreboardCommand {
    List,
    Add(String, ScoreboardCriterion, Option<SNBT>),
    Remove(String),
    SetDisplay(ScoreboardDisplaySlot, Option<String>),
    Modify(String, ScoreboardModification),
}

impl Display for ObjectivesScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectivesScoreboardCommand::List => f.write_str("list"),
            ObjectivesScoreboardCommand::Add(name, criterion, display_name) => {
                write!(f, "add {} {}", name, criterion)?;

                if let Some(display_name) = display_name {
                    write!(f, " {}", display_name)?;
                }

                Ok(())
            }
            ObjectivesScoreboardCommand::Remove(name) => {
                write!(f, "remove {}", name)
            }
            ObjectivesScoreboardCommand::SetDisplay(slot, name) => {
                write!(f, "setdisplay {}", slot)?;

                if let Some(name) = name {
                    write!(f, " {}", name)?;
                }

                Ok(())
            }
            ObjectivesScoreboardCommand::Modify(name, modification) => {
                write!(f, "modify {} {}", name, modification)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum PlayersDisplayScoreboardCommand {
    Name(PlayerScore, Option<SNBT>),
    NumberFormat(PlayerScore, Option<ScoreboardNumberFormat>),
}

impl Display for PlayersDisplayScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayersDisplayScoreboardCommand::Name(score, text) => {
                write!(f, "name {}", score)?;

                if let Some(text) = text {
                    write!(f, " {}", text)?;
                }

                Ok(())
            }
            PlayersDisplayScoreboardCommand::NumberFormat(score, number_format) => {
                write!(f, "numberformat {}", score)?;

                if let Some(number_format) = number_format {
                    write!(f, " {}", number_format)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum PlayersScoreboardCommand {
    List(Option<EntitySelector>),
    Get(PlayerScore),
    Set(PlayerScore, i32),
    Add(PlayerScore, i32),
    Remove(PlayerScore, i32),
    Reset(EntitySelector, Option<String>),
    Enable(PlayerScore),
    Operation(PlayerScore, ScoreOperationOperator, PlayerScore),
    Display(PlayersDisplayScoreboardCommand),
}

impl Display for PlayersScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayersScoreboardCommand::List(selector) => {
                f.write_str("list")?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;
                }

                Ok(())
            }
            PlayersScoreboardCommand::Get(score) => {
                write!(f, "get {}", score)
            }
            PlayersScoreboardCommand::Set(score, value) => {
                write!(f, "set {} {}", score, value)
            }
            PlayersScoreboardCommand::Add(score, value) => {
                write!(f, "add {} {}", score, value)
            }
            PlayersScoreboardCommand::Remove(score, value) => {
                write!(f, "remove {} {}", score, value)
            }
            PlayersScoreboardCommand::Reset(selector, objective) => {
                write!(f, "reset {}", selector)?;

                if let Some(objective) = objective {
                    write!(f, " {}", objective)?;
                }

                Ok(())
            }
            PlayersScoreboardCommand::Enable(score) => {
                write!(f, "enable {}", score)
            }
            PlayersScoreboardCommand::Operation(left, operator, right) => {
                write!(f, "operation {} {} {}", left, operator, right)
            }
            PlayersScoreboardCommand::Display(command) => {
                write!(f, "display {}", command)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ScoreboardCommand {
    Objectives(ObjectivesScoreboardCommand),
    Players(PlayersScoreboardCommand),
}

impl Display for ScoreboardCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreboardCommand::Objectives(command) => write!(f, "objectives {}", command),
            ScoreboardCommand::Players(command) => write!(f, "players {}", command),
        }
    }
}
