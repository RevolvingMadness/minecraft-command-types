use crate::command::item_source::ItemSource;
use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::item::ItemStack;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum LootTarget {
    Give(EntitySelector),
    Insert(Coordinates),
    Spawn(Coordinates),
    Replace(ItemSource, String, Option<i32>),
}

impl Display for LootTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LootTarget::Give(selector) => write!(f, "give {}", selector),
            LootTarget::Insert(coords) => write!(f, "insert {}", coords),
            LootTarget::Spawn(coords) => write!(f, "spawn {}", coords),
            LootTarget::Replace(item_source, slot, count) => {
                write!(f, "replace {} {}", item_source, slot)?;

                if let Some(count) = count {
                    write!(f, " {}", count)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum LootItemSource {
    Tool(ItemStack),
    Mainhand,
    Offhand,
}

impl Display for LootItemSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LootItemSource::Tool(tool) => tool.fmt(f),
            LootItemSource::Mainhand => f.write_str("mainhand"),
            LootItemSource::Offhand => f.write_str("offhand"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum LootSource {
    Fish(ResourceLocation, Coordinates, Option<LootItemSource>),
    Loot(ResourceLocation),
    Kill(EntitySelector),
    Mine(Coordinates, Option<LootItemSource>),
}

impl Display for LootSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LootSource::Fish(loot_table, pos, item_source) => {
                write!(f, "fish {} {}", loot_table, pos)?;

                if let Some(item_source) = item_source {
                    write!(f, " {}", item_source)?;
                }

                Ok(())
            }
            LootSource::Loot(loot_table) => {
                write!(f, "loot {}", loot_table)
            }
            LootSource::Kill(selector) => {
                write!(f, "kill {}", selector)
            }
            LootSource::Mine(coordinates, item_source) => {
                write!(f, "mine {}", coordinates)?;

                if let Some(item_source) = item_source {
                    write!(f, " {}", item_source)?;
                }

                Ok(())
            }
        }
    }
}
