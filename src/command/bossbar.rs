use crate::{
    command::{
        Command,
        enums::{
            bossbar_color::BossbarColor, bossbar_get_type::BossbarGetType,
            bossbar_style::BossbarStyle,
        },
    },
    entity_selector::EntitySelector,
    option_write_chain,
    resource_location::ResourceLocation,
    snbt::Snbt,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BossbarSetType {
    Color(BossbarColor),
    Max(i32),
    Name(Snbt),
    Players(Option<EntitySelector>),
    Style(BossbarStyle),
    Value(i32),
    Visible(bool),
}

impl Display for BossbarSetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BossbarCommand {
    Add(ResourceLocation, Snbt),
    Get(ResourceLocation, BossbarGetType),
    List,
    Remove(ResourceLocation),
    Set(ResourceLocation, BossbarSetType),
}

impl Display for BossbarCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
