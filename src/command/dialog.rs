use crate::entity_selector::EntitySelector;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_procederal_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DialogCommand {
    Show(EntitySelector, ResourceLocation),
    Clear(EntitySelector),
}

impl Display for DialogCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
