use crate::entity_selector::EntitySelector;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum EffectDuration {
    Duration(i32),
    Infinite,
}

impl Display for EffectDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectDuration::Duration(duration) => duration.fmt(f),
            EffectDuration::Infinite => f.write_str("infinite"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectCommand::Clear(selector, effect) => {
                f.write_str("clear")?;

                if let Some(selector) = selector {
                    write!(f, " {}", selector)?;

                    if let Some(effect) = effect {
                        write!(f, " {}", effect)?;
                    }
                }

                Ok(())
            }
            EffectCommand::Give(selector, effect, duration, amplifier, hide_particles) => {
                write!(f, "give {} {}", selector, effect)?;

                if let Some(duration) = duration {
                    write!(f, " {}", duration)?;

                    if let Some(amplifier) = amplifier {
                        write!(f, " {}", amplifier)?;

                        if let Some(hide_particles) = hide_particles {
                            write!(f, " {}", hide_particles)?;
                        }
                    }
                }

                Ok(())
            }
        }
    }
}
