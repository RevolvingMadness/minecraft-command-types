use crate::command::enums::attribute::AttributeAddModifier;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

type F32 = NotNan<f32>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum BaseAttributeCommand {
    /// Returns the base value of the specified attribute.
    Get(Option<F32>),
    /// Overwrites the base value of the specified attribute with the given value.
    Set(F32),
    /// Resets the base value of the specified attribute to its default value.
    Reset,
}

impl Display for BaseAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseAttributeCommand::Get(scale) => {
                f.write_str("get")?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            BaseAttributeCommand::Set(value) => write!(f, "set {}", value),
            BaseAttributeCommand::Reset => f.write_str("reset"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ModifierAttributeCommand {
    /// Adds an attribute modifier with the specified properties if no modifier with the same ID already existed.
    Add(ResourceLocation, F32, AttributeAddModifier),
    /// Removes the attribute modifier with the specified ID.
    Remove(ResourceLocation),
    /// Returns the value of the modifier with the specified ID.
    Get(ResourceLocation, Option<F32>),
}

impl Display for ModifierAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierAttributeCommand::Add(id, value, add_modifier) => {
                write!(f, "add {} {} {}", id, value, add_modifier)
            }
            ModifierAttributeCommand::Remove(id) => {
                write!(f, "remove {}", id)
            }
            ModifierAttributeCommand::Get(id, scale) => {
                write!(f, "value get {}", id)?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum AttributeCommand {
    /// Returns the total value of the specified attribute.
    Get(Option<F32>),
    Base(BaseAttributeCommand),
    Modifier(ModifierAttributeCommand),
}

impl Display for AttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeCommand::Get(scale) => {
                f.write_str("get")?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            AttributeCommand::Base(base_command) => {
                write!(f, "base {}", base_command)
            }
            AttributeCommand::Modifier(modifier_command) => {
                write!(f, "modifier {}", modifier_command)
            }
        }
    }
}
