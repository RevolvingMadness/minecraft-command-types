use crate::item::ItemStack;
use crate::snbt::SNBT;
use crate::{command::item_source::ItemSource, option_write_chain};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum ItemCommand {
    Modifier(SNBT),
    With(ItemStack, Option<i32>),
    From(ItemSource, String, Option<SNBT>),
}

impl Display for ItemCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Modifier(item) => item.fmt(f),
            Self::With(item, count) => {
                write!(f, "with {}", item)?;

                option_write_chain!(f, count);

                Ok(())
            }
            Self::From(source, slot, modifier) => {
                write!(f, "from {} {}", source, slot)?;

                option_write_chain!(f, modifier);

                Ok(())
            }
        }
    }
}
