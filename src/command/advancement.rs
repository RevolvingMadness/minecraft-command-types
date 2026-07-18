use crate::{option_write_chain, resource_location::ResourceLocation};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdvancementCommand {
    Everything,
    Only(ResourceLocation, Option<String>),
    From(ResourceLocation),
    Through(ResourceLocation),
    Until(ResourceLocation),
}

impl Display for AdvancementCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Everything => f.write_str("everything"),
            Self::Only(advancement, criterion) => {
                advancement.fmt(f)?;

                option_write_chain!(f, criterion);

                Ok(())
            }
            Self::From(advancement) | Self::Through(advancement) | Self::Until(advancement) => {
                advancement.fmt(f)
            }
        }
    }
}
