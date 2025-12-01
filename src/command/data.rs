use crate::coordinate::Coordinates;
use crate::entity_selector::EntitySelector;
use crate::nbt_path::NbtPath;
use crate::resource_location::ResourceLocation;
use crate::snbt::SNBT;
use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DataTarget {
    Block(Coordinates),
    Entity(EntitySelector),
    Storage(ResourceLocation),
}

impl Display for DataTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTarget::Block(coordinates) => {
                write!(f, "block {}", coordinates)
            }
            DataTarget::Entity(selector) => {
                write!(f, "entity {}", selector)
            }
            DataTarget::Storage(storage) => {
                write!(f, "storage {}", storage)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DataCommandModification {
    From(DataTarget, Option<NbtPath>),
    String(DataTarget, Option<NbtPath>, Option<i32>, Option<i32>),
    Value(SNBT),
}

impl Display for DataCommandModification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataCommandModification::From(source, path) => {
                write!(f, "from {}", source)?;

                if let Some(path) = path {
                    write!(f, " {}", path)?;
                }

                Ok(())
            }
            DataCommandModification::String(source, path, start, end) => {
                write!(f, "string {}", source)?;

                if let Some(path) = path {
                    write!(f, " {}", path)?;

                    if let Some(start) = start {
                        write!(f, " {}", start)?;

                        if let Some(end) = end {
                            write!(f, " {}", end)?;
                        }
                    }
                }

                Ok(())
            }
            DataCommandModification::Value(value) => {
                write!(f, "value {}", value)
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
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
            DataCommandModificationMode::Append => f.write_str("append"),
            DataCommandModificationMode::Prepend => f.write_str("prepend"),
            DataCommandModificationMode::Insert(index) => write!(f, "insert {}", index),
            DataCommandModificationMode::Merge => f.write_str("merge"),
            DataCommandModificationMode::Set => f.write_str("set"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum DataCommand {
    Get(DataTarget, Option<NbtPath>, Option<NotNan<f32>>),
    Merge(DataTarget, SNBT),
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
            DataCommand::Get(target, path, scale) => {
                target.fmt(f)?;

                if let Some(path) = path {
                    write!(f, " {}", path)?;

                    if let Some(scale) = scale {
                        write!(f, " {}", scale)?;
                    }
                }

                Ok(())
            }
            DataCommand::Merge(target, nbt) => {
                write!(f, "merge {} {}", target, nbt)
            }
            DataCommand::Modify(target, path, modification_mode, modification_command) => {
                write!(
                    f,
                    "modify {} {} {} {}",
                    target, path, modification_mode, modification_command
                )
            }
            DataCommand::Remove(target, path) => {
                write!(f, "remove {} {}", target, path)
            }
        }
    }
}
