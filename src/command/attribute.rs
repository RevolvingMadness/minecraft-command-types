use crate::command::enums::attribute::AttributeAddModifier;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_procedural_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

type F32 = NotNan<f32>;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum BaseAttributeCommand {
    Get(Option<F32>),
    Set(F32),
    Reset,
}

impl Display for BaseAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(scale) => {
                f.write_str("get")?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            Self::Set(value) => write!(f, "set {}", value),
            Self::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ModifierAttributeCommand {
    Add(ResourceLocation, F32, AttributeAddModifier),
    Remove(ResourceLocation),
    Get(ResourceLocation, Option<F32>),
}

impl Display for ModifierAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(id, value, add_modifier) => {
                write!(f, "add {} {} {}", id, value, add_modifier)
            }
            Self::Remove(id) => {
                write!(f, "remove {}", id)
            }
            Self::Get(id, scale) => {
                write!(f, "value get {}", id)?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum AttributeCommand {
    Get(Option<F32>),
    Base(BaseAttributeCommand),
    Modifier(ModifierAttributeCommand),
}

impl Display for AttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(scale) => {
                f.write_str("get")?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            Self::Base(base_command) => {
                write!(f, "base {}", base_command)
            }
            Self::Modifier(modifier_command) => {
                write!(f, "modifier {}", modifier_command)
            }
        }
    }
}
