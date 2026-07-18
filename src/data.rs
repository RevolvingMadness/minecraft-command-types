use crate::{
    command::{
        Command,
        data::{DataCommand, DataCommandModification, DataCommandModificationMode, DataTarget},
    },
    nbt_path::NbtPath,
    snbt::{Snbt, SnbtCompound},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Data {
    pub target: DataTarget,
    pub path: NbtPath,
}

impl Data {
    #[must_use]
    pub fn to_text_component(self) -> Snbt {
        let mut compound = SnbtCompound::new();

        compound.extend(self.target.to_snbt());

        compound.insert("nbt".to_owned(), Snbt::String(format!("{}", self.path)));

        Snbt::Compound(compound)
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
