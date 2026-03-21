use minecraft_command_types_derive::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

use crate::macroable::Macroable;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, HasMacro)]
pub struct WorldCoordinate {
    pub relative: bool,
    pub value: Option<Macroable<NotNan<f64>>>,
}

impl WorldCoordinate {
    #[inline]
    #[must_use]
    pub fn new(relative: bool, value: Option<Macroable<NotNan<f64>>>) -> Self {
        assert!(
            relative || value.is_some(),
            "A world coordinate must have a relative coordinate and/or have a value"
        );

        Self { relative, value }
    }

    #[inline]
    #[must_use]
    pub fn relative(value: Macroable<NotNan<f64>>) -> Self {
        Self::new(true, Some(value))
    }

    #[inline]
    #[must_use]
    pub fn relative_optional(value: Option<Macroable<NotNan<f64>>>) -> Self {
        Self::new(true, value)
    }

    #[inline]
    #[must_use]
    pub fn absolute(value: Macroable<NotNan<f64>>) -> Self {
        Self::new(false, Some(value))
    }

    #[inline]
    #[must_use]
    pub fn absolute_optional(value: Option<Macroable<NotNan<f64>>>) -> Self {
        Self::new(false, value)
    }

    #[inline]
    #[must_use]
    pub fn relative_zero() -> Self {
        Self::relative_optional(None)
    }

    #[inline]
    #[must_use]
    pub fn absolute_zero() -> Self {
        Self::absolute(Macroable::Regular(NotNan::new(0.0).unwrap()))
    }
}

impl Display for WorldCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.relative {
            f.write_str("~")?;
        }

        if let Some(value) = &self.value {
            value.fmt(f)?;
        }

        Ok(())
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

impl Coordinates {
    #[must_use]
    pub const fn new_world(x: WorldCoordinate, y: WorldCoordinate, z: WorldCoordinate) -> Self {
        Self::World(x, y, z)
    }

    #[inline]
    #[must_use]
    pub fn new_world_all_relative_zero() -> Self {
        Self::new_world(
            WorldCoordinate::relative_zero(),
            WorldCoordinate::relative_zero(),
            WorldCoordinate::relative_zero(),
        )
    }

    #[must_use]
    pub const fn new_local(
        x: Option<Macroable<NotNan<f64>>>,
        y: Option<Macroable<NotNan<f64>>>,
        z: Option<Macroable<NotNan<f64>>>,
    ) -> Self {
        Self::Local(x, y, z)
    }

    #[must_use]
    pub const fn new_local_zero() -> Self {
        Self::new_local(None, None, None)
    }
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
