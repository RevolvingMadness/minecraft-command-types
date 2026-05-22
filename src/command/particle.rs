use crate::command::{Command, enums::particle_display_type::ParticleDisplayType};
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::option_write_chain;
use minecraft_command_types_procedural_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ParticleCommand {
    Regular(String, Option<Coordinates>),
    Extra(
        String,
        Coordinates,
        Coordinates,
        NotNan<f32>,
        i32,
        Option<ParticleDisplayType>,
        Option<EntitySelector>,
    ),
}

impl Display for ParticleCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
