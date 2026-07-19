use crate::{
    command::Command,
    coordinate::Coordinates,
    entity_selector::EntitySelector,
    nbt_path::NbtPath,
    option_write_chain,
    resource_location::ResourceLocation,
    snbt::{Snbt, SnbtCompound},
    types::Float,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataTarget {
    Block(Coordinates),
    Entity(EntitySelector),
    Storage(ResourceLocation),
}

impl Display for DataTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(coordinates) => {
                write!(f, "block {}", coordinates)
            }
            Self::Entity(selector) => {
                write!(f, "entity {}", selector)
            }
            Self::Storage(storage) => {
                write!(f, "storage {}", storage)
            }
        }
    }
}

impl DataTarget {
    #[must_use]
    pub fn to_snbt(&self) -> SnbtCompound {
        let mut compound = SnbtCompound::new();

        match self {
            Self::Block(coordinates) => {
                compound.insert("source".to_owned(), Snbt::String("block".to_owned()));

                compound.insert("block".to_owned(), Snbt::String(format!("{}", coordinates)));
            }
            Self::Entity(selector) => {
                compound.insert("source".to_owned(), Snbt::String("entity".to_owned()));

                compound.insert("entity".to_owned(), Snbt::String(format!("{}", selector)));
            }
            Self::Storage(storage) => {
                compound.insert("source".to_owned(), Snbt::String("storage".to_owned()));

                compound.insert("storage".to_owned(), Snbt::String(format!("{}", storage)));
            }
        }

        compound
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataCommandModification {
    From(DataTarget, Option<NbtPath>),
    String(DataTarget, Option<NbtPath>, Option<i32>, Option<i32>),
    Value(Snbt),
}

impl Display for DataCommandModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::From(source, path) => {
                write!(f, "from {}", source)?;

                option_write_chain!(f, path);

                Ok(())
            }
            Self::String(source, path, start, end) => {
                write!(f, "string {}", source)?;

                option_write_chain!(f, path, start, end);

                Ok(())
            }
            Self::Value(value) => {
                write!(f, "value {}", value)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DataCommandModificationMode {
    Append,
    Prepend,
    Insert(i32),
    Merge,
    Set,
}

impl Display for DataCommandModificationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Append => f.write_str("append"),
            Self::Prepend => f.write_str("prepend"),
            Self::Insert(index) => write!(f, "insert {}", index),
            Self::Merge => f.write_str("merge"),
            Self::Set => f.write_str("set"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataCommand {
    Get(DataTarget, Option<NbtPath>, Option<Float>),
    Merge(DataTarget, Snbt),
    Modify(
        DataTarget,
        NbtPath,
        DataCommandModificationMode,
        DataCommandModification,
    ),
    Remove(DataTarget, NbtPath),
}

impl Display for DataCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get(target, path, scale) => {
                write!(f, "get {}", target)?;

                option_write_chain!(f, path, scale);

                Ok(())
            }
            Self::Merge(target, nbt) => {
                write!(f, "merge {} {}", target, nbt)
            }
            Self::Modify(target, path, modification_mode, modification_command) => {
                write!(
                    f,
                    "modify {} {} {} {}",
                    target, path, modification_mode, modification_command
                )
            }
            Self::Remove(target, path) => {
                write!(f, "remove {} {}", target, path)
            }
        }
    }
}

impl From<DataCommand> for Command {
    fn from(value: DataCommand) -> Self {
        Self::Data(value)
    }
}

impl DataCommand {
    #[must_use]
    pub fn has_side_effects(&self) -> bool {
        match self {
            Self::Get(..) => false,
            Self::Merge(_, Snbt::Compound(compound)) => !compound.is_empty(),
            Self::Merge(..) => true,
            Self::Modify(
                _,
                _,
                DataCommandModificationMode::Merge,
                DataCommandModification::Value(Snbt::Compound(compound)),
            ) => !compound.is_empty(),
            Self::Modify(..) => true,
            Self::Remove(..) => true,
        }
    }
}
