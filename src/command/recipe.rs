use crate::resource_location::ResourceLocation;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum RecipeType {
    All,
    Recipe(ResourceLocation),
}

impl Display for RecipeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeType::All => f.write_str("*"),
            RecipeType::Recipe(recipe) => recipe.fmt(f),
        }
    }
}
