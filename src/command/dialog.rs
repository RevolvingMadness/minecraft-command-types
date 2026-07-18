use crate::resource_location::ResourceLocation;
use crate::{command::Command, entity_selector::EntitySelector};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub enum DialogCommand {
    Show(EntitySelector, ResourceLocation),
    Clear(EntitySelector),
}

impl Display for DialogCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Show(selector, dialog) => {
                write!(f, "show {} {}", selector, dialog)
            }
            Self::Clear(selector) => {
                write!(f, "clear {}", selector)
            }
        }
    }
}

impl From<DialogCommand> for Command {
    fn from(value: DialogCommand) -> Self {
        Self::Dialog(value)
    }
}
