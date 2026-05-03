use std::{collections::BTreeMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{
    has_macro::HasMacro,
    snbt::{SNBT, SNBTString},
};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Macroable<T> {
    Regular(T),
    Macro(String),
}

pub trait RegularMacroableExt: Sized {
    fn regular_macroable(self) -> Macroable<Self>;
}

impl<T> RegularMacroableExt for T {
    fn regular_macroable(self) -> Macroable<Self> {
        Macroable::Regular(self)
    }
}

impl<T: HasMacro + Serialize> Serialize for Macroable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Regular(value) => value.serialize(serializer),
            Self::Macro(name) => serializer.serialize_str(&format!("$({})", name)),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Macroable<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;

        Ok(Self::Regular(value))
    }
}

impl<T> From<T> for Macroable<T> {
    fn from(value: T) -> Self {
        Self::Regular(value)
    }
}

impl<T> FromIterator<Macroable<T>> for Macroable<Vec<Macroable<T>>> {
    fn from_iter<I: IntoIterator<Item = Macroable<T>>>(iter: I) -> Self {
        let collected: Vec<Macroable<T>> = iter.into_iter().collect();

        if collected.is_empty() {
            Self::Regular(vec![])
        } else {
            Self::Regular(collected)
        }
    }
}

impl FromIterator<(SNBTString, Macroable<SNBT>)>
    for Macroable<BTreeMap<SNBTString, Macroable<SNBT>>>
{
    fn from_iter<I: IntoIterator<Item = (SNBTString, Macroable<SNBT>)>>(iter: I) -> Self {
        Self::Regular(iter.into_iter().collect())
    }
}

impl<T: Display> Display for Macroable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular(value) => value.fmt(f),
            Self::Macro(name) => write!(f, "$({})", name),
        }
    }
}

impl<T: HasMacro> HasMacro for Macroable<T> {
    fn has_macro(&self) -> bool {
        match self {
            Self::Regular(value) => value.has_macro(),
            Self::Macro(_) => true,
        }
    }

    fn has_macro_conflict(&self) -> bool {
        match self {
            Self::Regular(value) => value.has_macro_conflict(),
            Self::Macro(_) => false,
        }
    }
}

impl<T> Macroable<T> {
    #[must_use]
    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Macroable<R> {
        match self {
            Self::Regular(value) => Macroable::Regular(f(value)),
            Self::Macro(name) => Macroable::Macro(name),
        }
    }
}
