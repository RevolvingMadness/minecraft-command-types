use crate::command::Command;
use crate::entity_selector::EntitySelector;
use crate::nbt_path::NbtPath;
use crate::option_write_chain;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use crate::types::Float;
use crate::{coordinate::Coordinates, macroable::Macroable};
use minecraft_command_types_procedural_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DataTarget {
    Block(Coordinates),
    Entity(EntitySelector),
    Storage(ResourceLocation),
}

impl Display for DataTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DataCommandModification {
    From(DataTarget, Option<NbtPath>),
    String(DataTarget, Option<NbtPath>, Option<i32>, Option<i32>),
    Value(Macroable<SNBT>),
}

impl Display for DataCommandModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DataCommandModificationMode {
    Append,
    Prepend,
    Insert(i32),
    Merge,
    Set,
}

impl Display for DataCommandModificationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Append => f.write_str("append"),
            Self::Prepend => f.write_str("prepend"),
            Self::Insert(index) => write!(f, "insert {}", index),
            Self::Merge => f.write_str("merge"),
            Self::Set => f.write_str("set"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum DataCommand {
    Get(DataTarget, Option<NbtPath>, Option<Float>),
    Merge(DataTarget, Macroable<SNBT>),
    Modify(
        DataTarget,
        NbtPath,
        DataCommandModificationMode,
        DataCommandModification,
    ),
    Remove(DataTarget, NbtPath),
}

impl Display for DataCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            Self::Merge(_, Macroable::Regular(SNBT::Compound(compound))) => !compound.is_empty(),
            Self::Merge(..) => true,
            Self::Modify(
                _,
                _,
                DataCommandModificationMode::Merge,
                DataCommandModification::Value(Macroable::Regular(SNBT::Compound(compound))),
            ) => !compound.is_empty(),
            Self::Modify(..) => true,
            Self::Remove(..) => true,
        }
    }
}
