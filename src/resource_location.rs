use itertools::Itertools;
use nonempty::{nonempty, NonEmpty};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ResourceLocation {
    namespace: Option<String>,
    paths: NonEmpty<String>,
}

impl ResourceLocation {
    #[inline]
    #[must_use]
    pub fn new<T: ToString>(namespace: Option<T>, paths: NonEmpty<T>) -> Self {
        Self {
            namespace: namespace.map(|namespace| namespace.to_string()),
            paths: paths.map(|path| path.to_string()),
        }
    }

    #[inline]
    #[must_use]
    pub fn new_namespace_paths<T: ToString>(namespace: T, paths: NonEmpty<T>) -> Self {
        Self::new(Some(namespace), paths)
    }

    #[inline]
    #[must_use]
    pub fn new_namespace_path<T: ToString>(namespace: T, path: T) -> Self {
        Self::new_namespace_paths(namespace, nonempty![path])
    }

    #[inline]
    #[must_use]
    pub fn new_paths<T: ToString>(paths: NonEmpty<T>) -> Self {
        Self::new(None, paths)
    }

    #[inline]
    #[must_use]
    pub fn new_path<T: ToString>(path: T) -> Self {
        Self::new(None, nonempty![path])
    }
}

impl Display for ResourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(namespace) = &self.namespace {
            if *namespace != "minecraft".to_string() {
                write!(f, "{}:", namespace)?;
            }
        }

        self.paths.iter().join("/").fmt(f)
    }
}
