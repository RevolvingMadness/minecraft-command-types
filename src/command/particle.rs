use crate::{
    command::{Command, enums::particle_display_type::ParticleDisplayType},
    coordinate::Coordinates,
    entity_selector::EntitySelector,
    option_write_chain,
    types::Float,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParticleCommand {
    Regular(String, Option<Coordinates>),
    Extra(
        String,
        Coordinates,
        Coordinates,
        Float,
        i32,
        Option<ParticleDisplayType>,
        Option<EntitySelector>,
    ),
}

impl Display for ParticleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular(name, pos) => {
                name.fmt(f)?;

                option_write_chain!(f, pos);

                Ok(())
            }
            Self::Extra(name, pos, delta, speed, count, display_type, viewers) => {
                write!(f, "{} {} {} {} {}", name, pos, delta, speed, count)?;

                option_write_chain!(f, display_type, viewers);

                Ok(())
            }
        }
    }
}

impl From<ParticleCommand> for Command {
    fn from(value: ParticleCommand) -> Self {
        Self::Particle(value)
    }
}
