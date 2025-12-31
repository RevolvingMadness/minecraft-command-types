use nonempty::NonEmpty;
use std::collections::{BTreeMap, BTreeSet};

pub trait HasMacro {
    fn has_macro(&self) -> bool;
    fn has_macro_conflict(&self) -> bool;
}

#[macro_export]
macro_rules! impl_has_macro_false {
    ($($t:ty),*) => {
        $(
            impl ::minecraft_command_types::has_macro::HasMacro for $t {
                #[inline(always)]
                fn has_macro(&self) -> bool { false }

                fn has_macro_conflict(&self) -> bool { false }
            }
        )*
    };
}

impl_has_macro_false!(
    bool,
    i8,
    i16,
    i32,
    i64,
    String,
    ordered_float::NotNan<f32>,
    ordered_float::NotNan<f64>
);

impl<A: HasMacro, B: HasMacro> HasMacro for (A, B) {
    fn has_macro(&self) -> bool {
        self.0.has_macro() || self.1.has_macro()
    }

    fn has_macro_conflict(&self) -> bool {
        self.0.has_macro_conflict() || self.1.has_macro_conflict()
    }
}

impl<T: HasMacro> HasMacro for Option<T> {
    fn has_macro(&self) -> bool {
        self.as_ref().map(|t| t.has_macro()).unwrap_or(false)
    }

    fn has_macro_conflict(&self) -> bool {
        self.as_ref()
            .map(|t| t.has_macro_conflict())
            .unwrap_or(false)
    }
}

impl<T: HasMacro> HasMacro for Vec<T> {
    fn has_macro(&self) -> bool {
        self.iter().any(|t| t.has_macro())
    }

    fn has_macro_conflict(&self) -> bool {
        self.iter().any(|t| t.has_macro_conflict())
    }
}

impl<T: HasMacro> HasMacro for NonEmpty<T> {
    fn has_macro(&self) -> bool {
        self.iter().any(|t| t.has_macro())
    }

    fn has_macro_conflict(&self) -> bool {
        self.iter().any(|t| t.has_macro_conflict())
    }
}

impl<T: HasMacro> HasMacro for Box<T> {
    fn has_macro(&self) -> bool {
        self.as_ref().has_macro()
    }

    fn has_macro_conflict(&self) -> bool {
        self.as_ref().has_macro_conflict()
    }
}

impl<K, V: HasMacro> HasMacro for BTreeMap<K, V> {
    fn has_macro(&self) -> bool {
        self.values().any(|t| t.has_macro())
    }

    fn has_macro_conflict(&self) -> bool {
        self.values().any(|t| t.has_macro_conflict())
    }
}

impl<T: HasMacro> HasMacro for BTreeSet<T> {
    fn has_macro(&self) -> bool {
        self.iter().any(|t| t.has_macro())
    }

    fn has_macro_conflict(&self) -> bool {
        self.iter().any(|t| t.has_macro_conflict())
    }
}
