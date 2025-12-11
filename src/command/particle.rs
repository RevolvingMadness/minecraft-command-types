use crate::command::enums::particle_display_type::ParticleDisplayType;
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
            ParticleCommand::Regular(name, pos) => {
                name.fmt(f)?;

                if let Some(pos) = pos {
                    write!(f, " {}", pos)?;
                }

                Ok(())
            }
            ParticleCommand::Extra(name, pos, delta, speed, count, display_type, viewers) => {
                write!(f, "{} {} {} {} {}", name, pos, delta, speed, count)?;

                if let Some(display_type) = display_type {
                    write!(f, " {}", display_type)?;

                    if let Some(viewers) = viewers {
                        write!(f, " {}", viewers)?;
                    }
                }

                Ok(())
            }
        }
    }
}
