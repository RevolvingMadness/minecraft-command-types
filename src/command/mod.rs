mod permission_level;

use crate::command::permission_level::PermissionLevel;
use crate::entity_selector::EntitySelector;
use crate::enums::advancement_type::AdvancementType;
use crate::enums::attribute::AttributeAddModifier;
use crate::resource_location::ResourceLocation;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

type F32 = NotNan<f32>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AdvancementCommand {
    /// Adds or removes all loaded advancements.
    Everything,
    /// Adds or removes a single advancement or criterion.
    Only(ResourceLocation, Option<String>),
    /// Adds or removes an advancement and all its child advancements.
    /// Think of specifying everything from that advancement to the end.
    /// The exact order the operation is carried out in is `specified advancement > child > child's child > ...` When it operates on a child that branches, it iterates through all its children before continuing.
    From(ResourceLocation),
    /// Specifies an advancement, and adds or removes all its parent advancements, and all its child advancements.
    /// Think of specifying everything through the specified advancement, going both backward and forward.
    /// The exact order the operation is as if the command were executed with "until" specified, then with "from" specified: `parent > parent's parent > ... > root > specified advancement > child > child's child > ...`
    Through(ResourceLocation),
    /// Adds or removes an advancement and all its parent advancements until the root for addition/removal.
    /// Think of specifying everything from the start until that advancement.
    /// The exact order the operation is carried out in is: `parent > parent's parent > ... > root > specified advancement`.
    Until(ResourceLocation),
}

impl Display for AdvancementCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancementCommand::Everything => "everything".fmt(f),
            AdvancementCommand::Only(advancement, criterion) => {
                advancement.fmt(f)?;

                if let Some(criterion) = criterion {
                    write!(f, " {}", criterion)?;
                }

                Ok(())
            }
            AdvancementCommand::From(advancement) => advancement.fmt(f),
            AdvancementCommand::Through(advancement) => advancement.fmt(f),
            AdvancementCommand::Until(advancement) => advancement.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
                "get".fmt(f)?;

                if let Some(scale) = scale {
                    write!(f, " {}", scale)?;
                }

                Ok(())
            }
            BaseAttributeCommand::Set(value) => write!(f, "set {}", value),
            BaseAttributeCommand::Reset => "reset".fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
                "get".fmt(f)?;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Command {
    Advancement(AdvancementType, EntitySelector, AdvancementCommand),
    Attribute(EntitySelector, ResourceLocation, AttributeCommand),
}

impl Command {
    pub fn get_permission_level(&self) -> PermissionLevel {
        match self {
            Command::Advancement(..) | Command::Attribute(..) => {
                PermissionLevel::try_from(2).unwrap()
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Advancement(type_, selector, command) => {
                write!(f, "{} {} {}", type_, selector, command)
            }
            Command::Attribute(selector, attribute, command) => {
                write!(f, "{} {} {}", selector, attribute, command)
            }
        }
    }
}
