use crate::snbt::{Snbt, SnbtCompound, SnbtCompoundExt};
use std::fmt::{self, Display, Formatter};

fn escape_nbt_path_key(name: &str) -> String {
    let needs_quotes = name
        .chars()
        .any(|c| matches!(c, ' ' | '"' | '\'' | '[' | ']' | '.' | '{' | '}'));

    if needs_quotes {
        let escaped_content = name.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"{}\"", escaped_content)
    } else {
        name.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NbtPathNode {
    RootCompound(SnbtCompound),
    Named(String, Option<SnbtCompound>),
    Index(Option<Snbt>),
}

impl NbtPathNode {
    #[inline]
    #[must_use]
    pub const fn named(name: String) -> Self {
        Self::Named(name, None)
    }

    #[must_use]
    pub const fn can_dot_be_omitted(&self) -> bool {
        match self {
            Self::RootCompound(..) => false,
            Self::Named(..) => false,
            Self::Index(..) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NbtPath {
    pub nodes: Vec<NbtPathNode>,
}

impl NbtPath {
    #[must_use]
    pub fn with_node(mut self, node: NbtPathNode) -> Self {
        self.nodes.push(node);

        self
    }

    #[must_use]
    pub fn with_named_compound(mut self, compound: SnbtCompound) -> Self {
        if let Some(NbtPathNode::Named(_, inner_compound @ None)) = self.nodes.last_mut() {
            *inner_compound = Some(compound);
        }

        self
    }
}

impl Display for NbtPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootCompound(compound) => write!(f, "{}", (*compound).display_as_compound()),
            Self::Named(name, filter) => {
                f.write_str(&escape_nbt_path_key(name))?;

                if let Some(filter) = filter
                    && !filter.is_empty()
                {
                    write!(f, "{}", filter.display_as_compound())?;
                }

                Ok(())
            }
            Self::Index(Some(snbt)) => write!(f, "[{}]", snbt),
            Self::Index(None) => write!(f, "[]"),
        }
    }
}

impl Display for NbtPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            if i != 0 && !node.can_dot_be_omitted() {
                write!(f, ".")?;
            }

            write!(f, "{}", node)?;
        }

        Ok(())
    }
}
