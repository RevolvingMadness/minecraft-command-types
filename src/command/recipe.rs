use crate::resource_location::ResourceLocation;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecipeMode {
    Give,
    Take,
}

impl Display for RecipeMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Give => f.write_str("give"),
            Self::Take => f.write_str("take"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecipeType {
    All,
    Recipe(ResourceLocation),
}

impl Display for RecipeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => f.write_str("*"),
            Self::Recipe(recipe) => recipe.fmt(f),
        }
    }
}
