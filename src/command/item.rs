use crate::{command::item_source::ItemSource, item::ItemStack, option_write_chain, snbt::Snbt};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemCommand {
    Modifier(Snbt),
    With(ItemStack, Option<i32>),
    From(ItemSource, String, Option<Snbt>),
}

impl Display for ItemCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
