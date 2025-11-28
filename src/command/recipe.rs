use crate::has_macro::HasMacro;
use crate::resource_location::ResourceLocation;
use minecraft_command_types_proc_macros::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum RecipeType {
    All,
    Recipe(ResourceLocation),
}

impl Display for RecipeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeType::All => "*".fmt(f),
            RecipeType::Recipe(recipe) => recipe.fmt(f),
        }
    }
}
