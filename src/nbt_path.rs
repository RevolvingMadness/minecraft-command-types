use crate::has_macro::HasMacro;
use crate::macroable::Macroable;
use crate::snbt::{SNBT, SNBTString, fmt_snbt_compound};
use minecraft_command_types_procedural_macros::HasMacro;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub type SNBTCompound = BTreeMap<SNBTString, Macroable<SNBT>>;

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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum NbtPathNode {
    RootCompound(SNBTCompound),
    Named(SNBTString, Option<SNBTCompound>),
    Index(Option<Macroable<SNBT>>),
}

impl NbtPathNode {
    #[must_use]
    pub const fn named(name: SNBTString) -> Self {
        Self::Named(name, None)
    }

    #[must_use]
    pub const fn named_string(name: String) -> Self {
        Self::named(SNBTString(false, name))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct NbtPath(pub Vec<NbtPathNode>);

impl NbtPath {
    #[must_use]
    pub fn with_node(mut self, node: NbtPathNode) -> Self {
        self.0.push(node);

        self
    }

    #[must_use]
    pub fn with_named_compound(mut self, compound: SNBTCompound) -> Self {
        if let Some(NbtPathNode::Named(_, inner_compound @ None)) = self.0.last_mut() {
            *inner_compound = Some(compound);
        }

        self
    }

    #[inline]
    #[must_use]
    pub fn to_snbt_string(&self) -> SNBT {
        SNBT::String(Macroable::Regular(SNBTString(
            self.has_macro(),
            self.to_string(),
        )))
    }
}

impl Display for NbtPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RootCompound(compound) => fmt_snbt_compound(f, compound),
            Self::Named(SNBTString(_, name), filter) => {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for node in &self.0 {
            if !first && !matches!(node, NbtPathNode::Index(..)) {
                write!(f, ".")?;
            }
            first = false;
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}
