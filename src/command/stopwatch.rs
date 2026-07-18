use crate::{
    command::Command, option_write_chain, resource_location::ResourceLocation, types::Float,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StopwatchCommand {
    Create(ResourceLocation),
    Query(ResourceLocation, Option<Float>),
    Restart(ResourceLocation),
    Remove(ResourceLocation),
}

impl Display for StopwatchCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(location) => {
                write!(f, "create {}", location)
            }
            Self::Query(location, scale) => {
                write!(f, "query {}", location)?;

                option_write_chain!(f, scale);

                Ok(())
            }
            Self::Restart(location) => {
                write!(f, "restart {}", location)
            }
            Self::Remove(location) => {
                write!(f, "remove {}", location)
            }
        }
    }
}

impl From<StopwatchCommand> for Command {
    fn from(value: StopwatchCommand) -> Self {
        Self::Stopwatch(value)
    }
}

impl StopwatchCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Create(..) => true,
            Self::Query(..) => false,
            Self::Restart(..) => true,
            Self::Remove(..) => true,
        }
    }
}
