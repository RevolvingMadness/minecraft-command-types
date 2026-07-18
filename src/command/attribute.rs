use crate::{
    command::enums::attribute::AttributeAddModifier, option_write_chain,
    resource_location::ResourceLocation, types::Float,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BaseAttributeCommand {
    Get(Option<Float>),
    Set(Float),
    Reset,
}

impl Display for BaseAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModifierAttributeCommand {
    Add(ResourceLocation, Float, AttributeAddModifier),
    Remove(ResourceLocation),
    Get(ResourceLocation, Option<Float>),
}

impl Display for ModifierAttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttributeCommand {
    Get(Option<Float>),
    Base(BaseAttributeCommand),
    Modifier(ModifierAttributeCommand),
}

impl Display for AttributeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
