use crate::command::enums::bossbar_get_type::BossbarGetType;
use crate::command::enums::bossbar_style::BossbarStyle;
use crate::command::{Command, enums::bossbar_color::BossbarColor};
use crate::entity_selector::EntitySelector;
use crate::option_write_chain;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum BossbarSetType {
    Color(BossbarColor),
    Max(i32),
    Name(SNBT),
    Players(Option<EntitySelector>),
    Style(BossbarStyle),
    Value(i32),
    Visible(bool),
}

impl Display for BossbarSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Color(color) => write!(f, "color {}", color),
            Self::Max(max) => write!(f, "max {}", max),
            Self::Name(name) => write!(f, "name {}", name),
            Self::Players(players) => {
                f.write_str("players")?;

                option_write_chain!(f, players);

                Ok(())
            }
            Self::Style(style) => write!(f, "style {}", style),
            Self::Value(value) => write!(f, "value {}", value),
            Self::Visible(visible) => write!(f, "visible {}", visible),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum BossbarCommand {
    Add(ResourceLocation, SNBT),
    Get(ResourceLocation, BossbarGetType),
    List,
    Remove(ResourceLocation),
    Set(ResourceLocation, BossbarSetType),
}

impl Display for BossbarCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(id, name) => write!(f, "add {} {}", id, name),
            Self::Get(id, type_) => write!(f, "get {} {}", id, type_),
            Self::List => f.write_str("list"),
            Self::Remove(id) => write!(f, "remove {}", id),
            Self::Set(id, set_type) => write!(f, "set {} {}", id, set_type),
        }
    }
}

impl From<BossbarCommand> for Command {
    fn from(value: BossbarCommand) -> Self {
        Self::Bossbar(value)
    }
}

impl BossbarCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::Get(..) => false,
            Self::List => false,
            Self::Remove(..) => true,
            Self::Set(..) => true,
        }
    }
}
