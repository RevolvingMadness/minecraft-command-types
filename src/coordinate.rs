use minecraft_command_types_procedural_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter, Write};

use crate::macroable::Macroable;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum WorldCoordinate {
    Relative(Option<Macroable<NotNan<f64>>>),
    Absolute(Macroable<NotNan<f64>>),
}

impl Default for WorldCoordinate {
    fn default() -> Self {
        Self::RELATIVE_NONE
    }
}

impl WorldCoordinate {
    pub const RELATIVE_NONE: Self = Self::Relative(None);

    pub const ABSOLUTE_ZERO: Self =
        Self::Absolute(Macroable::Regular(unsafe { NotNan::new_unchecked(0.0) }));

    #[inline]
    #[must_use]
    pub const fn absolute_zero() -> Self {
        Self::ABSOLUTE_ZERO
    }
}

impl Display for WorldCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Relative(offset) => {
                f.write_char('~')?;

                if let Some(offset) = offset {
                    offset.fmt(f)?;
                }

                Ok(())
            }
            Self::Absolute(value) => value.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub enum Coordinates {
    World(WorldCoordinate, WorldCoordinate, WorldCoordinate),
    Local(
        Option<Macroable<NotNan<f64>>>,
        Option<Macroable<NotNan<f64>>>,
        Option<Macroable<NotNan<f64>>>,
    ),
}

impl Default for Coordinates {
    fn default() -> Self {
        Self::World(
            WorldCoordinate::default(),
            WorldCoordinate::default(),
            WorldCoordinate::default(),
        )
    }
}

impl Coordinates {
    pub const WORLD_RELATIVE_NONE: Self = Self::World(
        WorldCoordinate::RELATIVE_NONE,
        WorldCoordinate::RELATIVE_NONE,
        WorldCoordinate::RELATIVE_NONE,
    );

    pub const LOCAL_NONE: Self = Self::Local(None, None, None);
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::World(x, y, z) => {
                write!(f, "{} {} {}", x, y, z)
            }
            Self::Local(x, y, z) => {
                f.write_str("^")?;

                if let Some(x) = x {
                    x.fmt(f)?;
                }

                f.write_str(" ^")?;

                if let Some(y) = y {
                    y.fmt(f)?;
                }

                f.write_str(" ^")?;

                if let Some(z) = z {
                    z.fmt(f)?;
                }

                Ok(())
            }
        }
    }
}
