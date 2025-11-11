use crate::resource_location::ResourceLocation;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AdvancementCommand {
    /// Adds or removes all loaded advancements.
    Everything,
    /// Adds or removes a single advancement or criterion.
    Only(ResourceLocation, Option<String>),
    /// Adds or removes an advancement and all its child advancements.
    /// Think of specifying everything from that advancement to the end.
    /// The exact order the operation is carried out in is `specified advancement > child > child's child > ...` When it operates on a child that branches, it iterates through all its children before continuing.
    From(ResourceLocation),
    /// Specifies an advancement, and adds or removes all its parent advancements, and all its child advancements.
    /// Think of specifying everything through the specified advancement, going both backward and forward.
    /// The exact order the operation is as if the command were executed with "until" specified, then with "from" specified: `parent > parent's parent > ... > root > specified advancement > child > child's child > ...`
    Through(ResourceLocation),
    /// Adds or removes an advancement and all its parent advancements until the root for addition/removal.
    /// Think of specifying everything from the start until that advancement.
    /// The exact order the operation is carried out in is: `parent > parent's parent > ... > root > specified advancement`.
    Until(ResourceLocation),
}

impl Display for AdvancementCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancementCommand::Everything => "everything".fmt(f),
            AdvancementCommand::Only(advancement, criterion) => {
                advancement.fmt(f)?;

                if let Some(criterion) = criterion {
                    write!(f, " {}", criterion)?;
                }

                Ok(())
            }
            AdvancementCommand::From(advancement) => advancement.fmt(f),
            AdvancementCommand::Through(advancement) => advancement.fmt(f),
            AdvancementCommand::Until(advancement) => advancement.fmt(f),
        }
    }
}
