use crate::resource_location::ResourceLocation;
use minecraft_command_types_derive::HasMacro;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum AdvancementCommand {
    Everything,
    Only(ResourceLocation, Option<String>),
    From(ResourceLocation),
    Through(ResourceLocation),
    Until(ResourceLocation),
}

impl Display for AdvancementCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Everything => f.write_str("everything"),
            Self::Only(advancement, criterion) => {
                advancement.fmt(f)?;

                if let Some(criterion) = criterion {
                    write!(f, " {}", criterion)?;
                }

                Ok(())
            }
            Self::From(advancement) | Self::Through(advancement) | Self::Until(advancement) => {
                advancement.fmt(f)
            }
        }
    }
}
