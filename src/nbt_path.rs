use crate::snbt::{Snbt, SnbtCompound, fmt_snbt_compound};
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NbtPath(pub Vec<NbtPathNode>);

impl NbtPath {
    #[must_use]
    pub fn with_node(mut self, node: NbtPathNode) -> Self {
        self.0.push(node);

        self
    }

    #[must_use]
    pub fn with_named_compound(mut self, compound: SnbtCompound) -> Self {
        if let Some(NbtPathNode::Named(_, inner_compound @ None)) = self.0.last_mut() {
            *inner_compound = Some(compound);
        }

        self
    }

    #[inline]
    #[must_use]
    pub fn to_snbt_string(&self) -> Snbt {
        Snbt::String(self.to_string())
    }
}

impl Display for NbtPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootCompound(compound) => fmt_snbt_compound(f, compound),
            Self::Named(name, filter) => {
                f.write_str(&escape_nbt_path_key(name))?;

                if let Some(filter) = filter
                    && !filter.is_empty()
                {
                    fmt_snbt_compound(f, filter)?;
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
        for (i, node) in self.0.iter().enumerate() {
            if i != 0 && !matches!(node, NbtPathNode::Index(..)) {
                write!(f, ".")?;
            }

            write!(f, "{}", node)?;
        }

        Ok(())
    }
}
