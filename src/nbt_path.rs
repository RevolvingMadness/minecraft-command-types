use crate::has_macro::HasMacro;
use crate::snbt::{SNBT, SNBTString, fmt_snbt_compound};
use minecraft_command_types_derive::HasMacro;
use nonempty::NonEmpty;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub type SNBTCompound = BTreeMap<SNBTString, SNBT>;

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
    Index(Option<SNBT>),
}

impl NbtPathNode {
    pub fn named(name: SNBTString) -> Self {
        NbtPathNode::Named(name, None)
    }

    pub fn named_string(name: String) -> Self {
        NbtPathNode::named(SNBTString(false, name))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct NbtPath(pub NonEmpty<NbtPathNode>);

impl NbtPath {
    pub fn with_node(mut self, node: NbtPathNode) -> Self {
        self.0.push(node);

        self
    }

    pub fn with_named_compound(mut self, compound: SNBTCompound) -> Self {
        if let NbtPathNode::Named(_, inner_compound) = self.0.last_mut()
            && inner_compound.is_none()
        {
            *inner_compound = Some(compound);
        }

        self
    }

    pub fn to_snbt_string(&self) -> SNBT {
        SNBT::String(SNBTString(self.has_macro(), self.to_string()))
    }
}

impl Display for NbtPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtPathNode::RootCompound(compound) => fmt_snbt_compound(f, compound),
            NbtPathNode::Named(SNBTString(_, name), filter) => {
                f.write_str(&escape_nbt_path_key(name))?;

                if let Some(filter) = filter
                    && !filter.is_empty()
                {
                    fmt_snbt_compound(f, filter)?;
                }
                Ok(())
            }
            NbtPathNode::Index(Some(snbt)) => write!(f, "[{}]", snbt),
            NbtPathNode::Index(None) => write!(f, "[]"),
        }
    }
}

impl Display for NbtPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for node in &self.0 {
            if !first && !matches!(node, NbtPathNode::Index(_)) {
                write!(f, ".")?;
            }
            first = false;
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}
