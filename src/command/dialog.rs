use crate::{
    command::Command, entity_selector::EntitySelector, resource_location::ResourceLocation,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
