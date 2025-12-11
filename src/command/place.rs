use crate::command::enums::template_mirror::TemplateMirror;
use crate::command::enums::template_rotation::TemplateRotation;
use crate::coordinate::Coordinates;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum PlaceCommand {
    Feature(ResourceLocation, Option<Coordinates>),
    Jigsaw(ResourceLocation, ResourceLocation, i32, Option<Coordinates>),
    Structure(ResourceLocation, Option<Coordinates>),
    Template(
        ResourceLocation,
        Option<Coordinates>,
        Option<TemplateRotation>,
        Option<TemplateMirror>,
        Option<NotNan<f32>>,
        Option<i32>,
        Option<bool>,
    ),
}

impl Display for PlaceCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceCommand::Feature(feature, pos) => {
                write!(f, "feature {}", feature)?;

                if let Some(pos) = pos {
                    write!(f, " {}", pos)?;
                }

                Ok(())
            }
            PlaceCommand::Jigsaw(pool, target, max_depth, position) => {
                write!(f, "jigsaw {} {} {}", pool, target, max_depth)?;

                if let Some(position) = position {
                    write!(f, " {}", position)?;
                }

                Ok(())
            }
            PlaceCommand::Structure(structure, pos) => {
                write!(f, "structure {}", structure)?;

                if let Some(pos) = pos {
                    write!(f, " {}", pos)?;
                }

                Ok(())
            }
            PlaceCommand::Template(template, pos, rotation, mirror, integrity, seed, strict) => {
                write!(f, "template {}", template)?;

                if let Some(pos) = pos {
                    write!(f, " {}", pos)?;

                    if let Some(rotation) = rotation {
                        write!(f, " {}", rotation)?;

                        if let Some(mirror) = mirror {
                            write!(f, " {}", mirror)?;

                            if let Some(integrity) = integrity {
                                write!(f, " {}", integrity)?;

                                if let Some(seed) = seed {
                                    write!(f, " {}", seed)?;

                                    if let Some(strict) = strict {
                                        write!(f, " {}", strict)?;
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }
}
