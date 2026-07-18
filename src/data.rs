use minecraft_command_types_procedural_macros::HasMacro;

use crate::{
    command::{
        Command,
        data::{DataCommand, DataCommandModification, DataCommandModificationMode, DataTarget},
    },
    macroable::RegularMacroableExt,
    nbt_path::{NbtPath, SNBTCompound},
    snbt::{SNBT, SNBTString},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, HasMacro)]
pub struct Data {
    pub target: DataTarget,
    pub path: NbtPath,
}

impl Data {
    #[must_use]
    pub fn to_text_component(self) -> SNBT {
        let mut compound = SNBTCompound::new();

        compound.extend(self.target.to_snbt());

        compound.insert(
            SNBTString(false, "nbt".to_owned()),
            SNBT::String(SNBTString(false, format!("{}", self.path))).regular_macroable(),
        );

        SNBT::Compound(compound)
    }

    #[inline]
    #[must_use]
    pub fn get(self) -> Command {
        Command::Data(DataCommand::Get(self.target, Some(self.path), None))
    }

    #[inline]
    #[must_use]
    pub fn set<P: Into<Option<NbtPath>>>(self, target: DataTarget, path: P) -> Command {
        let path = path.into();

        Command::Data(DataCommand::Modify(
            self.target,
            self.path,
            DataCommandModificationMode::Set,
            DataCommandModification::From(target, path),
        ))
    }
}
