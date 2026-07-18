use crate::{
    command::item_source::ItemSource, coordinate::Coordinates, entity_selector::EntitySelector,
    item::ItemStack, option_write_chain, resource_location::ResourceLocation,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LootTarget {
    Give(EntitySelector),
    Insert(Coordinates),
    Spawn(Coordinates),
    Replace(ItemSource, String, Option<i32>),
}

impl Display for LootTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Give(selector) => write!(f, "give {}", selector),
            Self::Insert(coords) => write!(f, "insert {}", coords),
            Self::Spawn(coords) => write!(f, "spawn {}", coords),
            Self::Replace(item_source, slot, count) => {
                write!(f, "replace {} {}", item_source, slot)?;

                option_write_chain!(f, count);

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LootItemSource {
    Tool(ItemStack),
    Mainhand,
    Offhand,
}

impl Display for LootItemSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tool(tool) => tool.fmt(f),
            Self::Mainhand => f.write_str("mainhand"),
            Self::Offhand => f.write_str("offhand"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LootSource {
    Fish(ResourceLocation, Coordinates, Option<LootItemSource>),
    Loot(ResourceLocation),
    Kill(EntitySelector),
    Mine(Coordinates, Option<LootItemSource>),
}

impl Display for LootSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fish(loot_table, pos, item_source) => {
                write!(f, "fish {} {}", loot_table, pos)?;

                option_write_chain!(f, item_source);

                Ok(())
            }
            Self::Loot(loot_table) => {
                write!(f, "loot {}", loot_table)
            }
            Self::Kill(selector) => {
                write!(f, "kill {}", selector)
            }
            Self::Mine(coordinates, item_source) => {
                write!(f, "mine {}", coordinates)?;

                option_write_chain!(f, item_source);

                Ok(())
            }
        }
    }
}
