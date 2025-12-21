use crate::snbt::{SNBT, fmt_snbt_compound};
use minecraft_command_types_derive::HasMacro;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub type SNBTCompound = BTreeMap<String, SNBT>;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum NbtPathNode {
    RootCompound(SNBTCompound),
    Named(String, Option<SNBTCompound>),
    Index(Option<SNBT>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct NbtPath(pub Vec<NbtPathNode>);

impl NbtPath {
    pub fn with_node(mut self, node: NbtPathNode) -> Self {
        self.0.push(node);

        self
    }
}

impl Display for NbtPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtPathNode::RootCompound(compound) => fmt_snbt_compound(f, compound),
            NbtPathNode::Named(name, filter) => {
                write!(f, "{}", escape_nbt_path_key(name))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snbt::SNBT;
    use std::collections::BTreeMap;

    fn snbt_string(s: &str) -> SNBT {
        SNBT::String(s.to_string())
    }

    fn compound(pairs: Vec<(&str, SNBT)>) -> SNBTCompound {
        let mut map = BTreeMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v);
        }
        map
    }

    #[test]
    fn test_example_1() {
        let path = NbtPath(vec![
            NbtPathNode::Named("foo".to_string(), None),
            NbtPathNode::Named("bar".to_string(), None),
            NbtPathNode::Index(Some(SNBT::Integer(0))),
            NbtPathNode::Named("A [crazy name]!".to_string(), None),
            NbtPathNode::Named("baz".to_string(), None),
        ]);

        assert_eq!(path.to_string(), r#"foo.bar[0]."A [crazy name]!".baz"#);
    }

    #[test]
    fn test_example_2() {
        let path = NbtPath(vec![
            NbtPathNode::Named("Items".to_string(), None),
            NbtPathNode::Index(Some(SNBT::Integer(1))),
            NbtPathNode::Named("components".to_string(), None),
            NbtPathNode::Named("minecraft:written_book_content".to_string(), None),
            NbtPathNode::Named("pages".to_string(), None),
            NbtPathNode::Index(Some(SNBT::Integer(3))),
            NbtPathNode::Named("raw".to_string(), None),
        ]);

        assert_eq!(
            path.to_string(),
            r#"Items[1].components.minecraft:written_book_content.pages[3].raw"#
        );
    }

    #[test]
    fn test_root_compound_and_filters() {
        let path = NbtPath(vec![NbtPathNode::RootCompound(compound(vec![(
            "foo",
            snbt_string("4.0f"),
        )]))]);

        assert_eq!(path.to_string(), r#"{"foo":"4.0f"}"#);

        let path2 = NbtPath(vec![
            NbtPathNode::Named(
                "foo".to_string(),
                Some(compound(vec![("bar", snbt_string("baz"))])),
            ),
            NbtPathNode::Named("bar".to_string(), None),
        ]);

        assert_eq!(path2.to_string(), r#"foo{"bar":"baz"}.bar"#);
    }

    #[test]
    fn test_index_all() {
        let path = NbtPath(vec![
            NbtPathNode::Named("foo".to_string(), None),
            NbtPathNode::Named("bar".to_string(), None),
            NbtPathNode::Index(None),
            NbtPathNode::Named("baz".to_string(), None),
        ]);

        assert_eq!(path.to_string(), r#"foo.bar[].baz"#);
    }

    #[test]
    fn test_complex_escaping_with_new_rules() {
        let path_with_quotes = NbtPath(vec![NbtPathNode::Named(
            "key with \"quotes\"".to_string(),
            None,
        )]);
        assert_eq!(path_with_quotes.to_string(), r#""key with \"quotes\"""#);

        let path_with_dot = NbtPath(vec![NbtPathNode::Named("key.with.dot".to_string(), None)]);
        assert_eq!(path_with_dot.to_string(), r#""key.with.dot""#);

        let path_with_slash = NbtPath(vec![NbtPathNode::Named(
            "key with \\ backslash".to_string(),
            None,
        )]);
        assert_eq!(path_with_slash.to_string(), r#""key with \\ backslash""#);
    }
}
