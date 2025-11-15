use crate::entity_selector::EntitySelector;
use crate::has_macro::HasMacro;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DialogCommand {
    Show(EntitySelector, ResourceLocation),
    Clear(EntitySelector),
}

impl Display for DialogCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogCommand::Show(selector, dialog) => {
                write!(f, "show {} {}", selector, dialog)
            }
            DialogCommand::Clear(selector) => {
                write!(f, "clear {}", selector)
            }
        }
    }
}
