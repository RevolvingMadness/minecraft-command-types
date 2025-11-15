use crate::has_macro::HasMacro;
use crate::snbt::SNBT;
use minecraft_command_types_proc_macros::HasMacro;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

type Compound = BTreeMap<String, SNBT>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum NbtPathNode {
    RootCompound(Compound),
    Named(String, Option<Compound>),
    Index(Option<SNBT>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct NbtPath(Vec<NbtPathNode>);

impl Display for NbtPathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NbtPathNode::RootCompound(compound) => {
                if compound.is_empty() {
                    write!(f, "{{}}")
                } else {
                    let mut first = true;
                    write!(f, "{{")?;
                    for (k, v) in compound {
                        if !first {
                            write!(f, ",")?;
                        }
                        first = false;
                        if k.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
                            write!(f, "\"{}\":{}", k, v)?;
                        } else {
                            write!(f, "{}:{}", k, v)?;
                        }
                    }
                    write!(f, "}}")
                }
            }
            NbtPathNode::Named(name, filter) => {
                let needs_quotes = name.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_');
                if needs_quotes {
                    write!(f, "\"{}\"", name)?;
                } else {
                    write!(f, "{}", name)?;
                }

                if let Some(comp) = filter {
                    if !comp.is_empty() {
                        write!(f, "{{")?;
                        let mut first = true;
                        for (k, v) in comp {
                            if !first {
                                write!(f, ",")?;
                            }
                            first = false;
                            if k.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
                                write!(f, "\"{}\":{}", k, v)?;
                            } else {
                                write!(f, "{}:{}", k, v)?;
                            }
                        }
                        write!(f, "}}")?;
                    }
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
            if !first {
                if !matches!(node, NbtPathNode::Index(_)) {
                    write!(f, ".")?;
                }
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
        // Helper to create SNBT string nodes
        SNBT::String(s.to_string())
    }

    fn compound(pairs: Vec<(&str, SNBT)>) -> Compound {
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
            NbtPathNode::Index(Some(snbt_string("0"))),
            NbtPathNode::Named("A [crazy name]!".to_string(), None),
            NbtPathNode::Named("baz".to_string(), None),
        ]);

        assert_eq!(path.to_string(), r#"foo.bar[0]."A [crazy name]!".baz"#);
    }

    #[test]
    fn test_example_2() {
        let path = NbtPath(vec![
            NbtPathNode::Named("Items".to_string(), None),
            NbtPathNode::Index(Some(snbt_string("1"))),
            NbtPathNode::Named(
                "components.minecraft:written_book_content.pages".to_string(),
                None,
            ),
            NbtPathNode::Index(Some(snbt_string("3"))),
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

        assert_eq!(path.to_string(), r#"{foo:"4.0f"}"#);

        let path2 = NbtPath(vec![
            NbtPathNode::Named(
                "foo".to_string(),
                Some(compound(vec![("bar", snbt_string("baz"))])),
            ),
            NbtPathNode::Named("bar".to_string(), None),
        ]);

        assert_eq!(path2.to_string(), r#"foo{bar:"baz"}.bar"#);
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
}
