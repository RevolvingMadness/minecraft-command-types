use crate::{
    command::{
        Command,
        data::{DataCommand, DataCommandModification, DataCommandModificationType, DataTarget},
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

        compound.extend(self.target.to_text_component());

        compound.insert("nbt".to_owned(), Snbt::String(format!("{}", self.path)));

        Snbt::Compound(compound)
    }

    #[inline]
    #[must_use]
    pub fn get(self) -> Command {
        Command::Data(DataCommand::Get {
            target: self.target,
            path: Some(self.path),
            scale: None,
        })
    }

    #[inline]
    #[must_use]
    pub fn modify(
        self,
        modification_type: DataCommandModificationType,
        modification: DataCommandModification,
    ) -> Command {
        Command::Data(DataCommand::Modify {
            target: self.target,
            path: self.path,
            modification_type,
            modification,
        })
    }

    #[inline]
    #[must_use]
    pub fn modify_from<P: Into<Option<NbtPath>>>(
        self,
        type_: DataCommandModificationType,
        target: DataTarget,
        path: P,
    ) -> Command {
        let path = path.into();

        self.modify(type_, DataCommandModification::From { target, path })
    }

    #[inline]
    #[must_use]
    pub fn modify_string<P: Into<Option<NbtPath>>, S: Into<Option<i32>>, E: Into<Option<i32>>>(
        self,
        type_: DataCommandModificationType,
        target: DataTarget,
        path: P,
        start: S,
        end: E,
    ) -> Command {
        let path = path.into();
        let start = start.into();
        let end = end.into();

        self.modify(
            type_,
            DataCommandModification::String {
                target,
                path,
                start,
                end,
            },
        )
    }

    #[must_use]
    pub fn modify_value(self, type_: DataCommandModificationType, value: Snbt) -> Command {
        self.modify(type_, DataCommandModification::Value(value))
    }

    #[inline]
    #[must_use]
    pub fn set(self, modification: DataCommandModification) -> Command {
        self.modify(DataCommandModificationType::Set, modification)
    }

    #[inline]
    #[must_use]
    pub fn set_from<P: Into<Option<NbtPath>>>(self, target: DataTarget, path: P) -> Command {
        let path = path.into();

        self.set(DataCommandModification::From { target, path })
    }

    #[inline]
    #[must_use]
    pub fn set_string<P: Into<Option<NbtPath>>, S: Into<Option<i32>>, E: Into<Option<i32>>>(
        self,
        target: DataTarget,
        path: P,
        start: S,
        end: E,
    ) -> Command {
        let path = path.into();
        let start = start.into();
        let end = end.into();

        self.set(DataCommandModification::String {
            target,
            path,
            start,
            end,
        })
    }

    #[must_use]
    pub fn set_value(self, value: Snbt) -> Command {
        self.set(DataCommandModification::Value(value))
    }

    #[inline]
    #[must_use]
    pub fn append(self, modification: DataCommandModification) -> Command {
        self.modify(DataCommandModificationType::Append, modification)
    }

    #[inline]
    #[must_use]
    pub fn append_from<P: Into<Option<NbtPath>>>(self, target: DataTarget, path: P) -> Command {
        let path = path.into();

        self.append(DataCommandModification::From { target, path })
    }

    #[inline]
    #[must_use]
    pub fn append_string<P: Into<Option<NbtPath>>, S: Into<Option<i32>>, E: Into<Option<i32>>>(
        self,
        target: DataTarget,
        path: P,
        start: S,
        end: E,
    ) -> Command {
        let path = path.into();
        let start = start.into();
        let end = end.into();

        self.append(DataCommandModification::String {
            target,
            path,
            start,
            end,
        })
    }

    #[must_use]
    pub fn append_value(self, value: Snbt) -> Command {
        self.append(DataCommandModification::Value(value))
    }

    #[inline]
    #[must_use]
    pub fn prepend(self, modification: DataCommandModification) -> Command {
        self.modify(DataCommandModificationType::Prepend, modification)
    }

    #[inline]
    #[must_use]
    pub fn prepend_from<P: Into<Option<NbtPath>>>(self, target: DataTarget, path: P) -> Command {
        let path = path.into();

        self.prepend(DataCommandModification::From { target, path })
    }

    #[inline]
    #[must_use]
    pub fn prepend_string<P: Into<Option<NbtPath>>, S: Into<Option<i32>>, E: Into<Option<i32>>>(
        self,
        target: DataTarget,
        path: P,
        start: S,
        end: E,
    ) -> Command {
        let path = path.into();
        let start = start.into();
        let end = end.into();

        self.prepend(DataCommandModification::String {
            target,
            path,
            start,
            end,
        })
    }

    #[must_use]
    pub fn prepend_value(self, value: Snbt) -> Command {
        self.prepend(DataCommandModification::Value(value))
    }

    #[inline]
    #[must_use]
    pub fn merge(self, modification: DataCommandModification) -> Command {
        self.modify(DataCommandModificationType::Merge, modification)
    }

    #[inline]
    #[must_use]
    pub fn merge_from<P: Into<Option<NbtPath>>>(self, target: DataTarget, path: P) -> Command {
        let path = path.into();

        self.merge(DataCommandModification::From { target, path })
    }

    #[inline]
    #[must_use]
    pub fn merge_string<P: Into<Option<NbtPath>>, S: Into<Option<i32>>, E: Into<Option<i32>>>(
        self,
        target: DataTarget,
        path: P,
        start: S,
        end: E,
    ) -> Command {
        let path = path.into();
        let start = start.into();
        let end = end.into();

        self.merge(DataCommandModification::String {
            target,
            path,
            start,
            end,
        })
    }

    #[must_use]
    pub fn merge_value(self, value: Snbt) -> Command {
        self.merge(DataCommandModification::Value(value))
    }

    #[inline]
    #[must_use]
    pub fn insert(self, index: i32, modification: DataCommandModification) -> Command {
        self.modify(DataCommandModificationType::Insert { index }, modification)
    }

    #[inline]
    #[must_use]
    pub fn insert_from<P: Into<Option<NbtPath>>>(
        self,
        index: i32,
        target: DataTarget,
        path: P,
    ) -> Command {
        let path = path.into();

        self.insert(index, DataCommandModification::From { target, path })
    }

    #[inline]
    #[must_use]
    pub fn insert_string<P: Into<Option<NbtPath>>, S: Into<Option<i32>>, E: Into<Option<i32>>>(
        self,
        index: i32,
        target: DataTarget,
        path: P,
        start: S,
        end: E,
    ) -> Command {
        let path = path.into();
        let start = start.into();
        let end = end.into();

        self.insert(
            index,
            DataCommandModification::String {
                target,
                path,
                start,
                end,
            },
        )
    }

    #[must_use]
    pub fn insert_value(self, index: i32, value: Snbt) -> Command {
        self.insert(index, DataCommandModification::Value(value))
    }
}
