use crate::command::item_source::ItemSource;
use crate::item::ItemStack;
use crate::snbt::SNBT;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum ItemCommand {
    Modifier(SNBT),
    With(ItemStack, Option<i32>),
    From(ItemSource, String, Option<SNBT>),
}

impl Display for ItemCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemCommand::Modifier(item) => item.fmt(f),
            ItemCommand::With(item, count) => {
                write!(f, "with {}", item)?;

                if let Some(count) = count {
                    write!(f, " {}", count)?;
                }

                Ok(())
            }
            ItemCommand::From(source, slot, modifier) => {
                write!(f, "from {} {}", source, slot)?;

                if let Some(modifier) = modifier {
                    write!(f, " {}", modifier)?;
                }

                Ok(())
            }
        }
    }
}
