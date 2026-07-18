use crate::command::enums::template_rotation::TemplateRotation;
use crate::command::{Command, enums::template_mirror::TemplateMirror};
use crate::coordinate::Coordinates;
use crate::option_write_chain;
use crate::resource_location::ResourceLocation;
use crate::types::Float;
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum PlaceCommand {
    Feature(ResourceLocation, Option<Coordinates>),
    Jigsaw(ResourceLocation, ResourceLocation, i32, Option<Coordinates>),
    Structure(ResourceLocation, Option<Coordinates>),
    Template(
        ResourceLocation,
        Option<Coordinates>,
        Option<TemplateRotation>,
        Option<TemplateMirror>,
        Option<Float>,
        Option<i32>,
        Option<bool>,
    ),
}

impl Display for PlaceCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Feature(feature, pos) => {
                write!(f, "feature {}", feature)?;

                option_write_chain!(f, pos);

                Ok(())
            }
            Self::Jigsaw(pool, target, max_depth, position) => {
                write!(f, "jigsaw {} {} {}", pool, target, max_depth)?;

                option_write_chain!(f, position);

                Ok(())
            }
            Self::Structure(structure, pos) => {
                write!(f, "structure {}", structure)?;

                option_write_chain!(f, pos);

                Ok(())
            }
            Self::Template(template, pos, rotation, mirror, integrity, seed, strict) => {
                write!(f, "template {}", template)?;

                option_write_chain!(f, pos, rotation, mirror, integrity, seed, strict);

                Ok(())
            }
        }
    }
}

impl From<PlaceCommand> for Command {
    fn from(value: PlaceCommand) -> Self {
        Self::Place(value)
    }
}
