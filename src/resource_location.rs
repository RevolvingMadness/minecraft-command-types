use crate::has_macro::HasMacro;
use itertools::Itertools;
use minecraft_command_types_proc_macros::HasMacro;
use nonempty::{nonempty, NonEmpty};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, HasMacro)]
pub struct ResourceLocation {
    pub is_tag: bool,
    pub namespace: Option<String>,
    pub paths: NonEmpty<String>,
}

impl ResourceLocation {
    #[inline]
    #[must_use]
    pub fn new<T: ToString>(is_tag: bool, namespace: Option<T>, paths: NonEmpty<T>) -> Self {
        Self {
            is_tag,
            namespace: namespace.map(|namespace| namespace.to_string()),
            paths: paths.map(|path| path.to_string()),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_namespace_paths<T: ToString>(namespace: T, paths: NonEmpty<T>) -> Self {
        Self::new(false, Some(namespace), paths)
    }

    #[inline]
    #[must_use]
    pub fn new_namespace_path<T: ToString>(namespace: T, path: T) -> Self {
        Self::new_namespace_paths(namespace, nonempty![path])
    }

    #[inline]
    #[must_use]
    pub fn new_paths<T: ToString>(paths: NonEmpty<T>) -> Self {
        Self::new(false, None, paths)
    }

    #[inline]
    #[must_use]
    pub fn new_path<T: ToString>(path: T) -> Self {
        Self::new_paths(nonempty![path])
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_tag {
            "#".fmt(f)?;
        }

        if let Some(namespace) = &self.namespace {
            if *namespace != "minecraft".to_string() {
                write!(f, "{}:", namespace)?;
            }
        }

        self.paths.iter().join("/").fmt(f)
    }
}
