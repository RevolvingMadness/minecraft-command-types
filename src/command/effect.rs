use crate::{
    command::Command, entity_selector::EntitySelector, option_write_chain,
    resource_location::ResourceLocation,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectDuration {
    Duration(i32),
    Infinite,
}

impl Display for EffectDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duration(duration) => duration.fmt(f),
            Self::Infinite => f.write_str("infinite"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EffectCommand {
    Clear(Option<EntitySelector>, Option<ResourceLocation>),
    Give(
        EntitySelector,
        ResourceLocation,
        Option<EffectDuration>,
        Option<i32>,
        Option<bool>,
    ),
}

impl Display for EffectCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear(selector, effect) => {
                f.write_str("clear")?;

                option_write_chain!(f, selector, effect);

                Ok(())
            }
            Self::Give(selector, effect, duration, amplifier, hide_particles) => {
                write!(f, "give {} {}", selector, effect)?;

                option_write_chain!(f, duration, amplifier, hide_particles);

                Ok(())
            }
        }
    }
}

impl From<EffectCommand> for Command {
    fn from(value: EffectCommand) -> Self {
        Self::Effect(value)
    }
}
