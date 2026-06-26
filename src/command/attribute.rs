use crate::resource_location::ResourceLocation;
use crate::{command::enums::attribute::AttributeAddModifier, option_write_chain};
use minecraft_command_types_procedural_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum BaseAttributeCommand {
    Get(Option<NotNan<f32>>),
    Set(NotNan<f32>),
    Reset,
}

impl Display for BaseAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(scale) => {
                f.write_str("get")?;

                option_write_chain!(f, scale);

                Ok(())
            }
            Self::Set(value) => write!(f, "set {}", value),
            Self::Reset => f.write_str("reset"),
        }
    }
}

impl BaseAttributeCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Get(..) => false,
            Self::Set(..) => true,
            Self::Reset => true,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ModifierAttributeCommand {
    Add(ResourceLocation, NotNan<f32>, AttributeAddModifier),
    Remove(ResourceLocation),
    Get(ResourceLocation, Option<NotNan<f32>>),
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

                option_write_chain!(f, scale);

                Ok(())
            }
        }
    }
}

impl ModifierAttributeCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Add(..) => true,
            Self::Remove(..) => true,
            Self::Get(..) => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum AttributeCommand {
    Get(Option<NotNan<f32>>),
    Base(BaseAttributeCommand),
    Modifier(ModifierAttributeCommand),
}

impl Display for AttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get(scale) => {
                f.write_str("get")?;

                option_write_chain!(f, scale);

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

impl AttributeCommand {
    #[must_use]
    pub const fn has_side_effects(&self) -> bool {
        match self {
            Self::Get(..) => false,
            Self::Base(command) => command.has_side_effects(),
            Self::Modifier(command) => command.has_side_effects(),
        }
    }
}
