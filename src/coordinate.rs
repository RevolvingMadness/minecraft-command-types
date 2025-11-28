use minecraft_command_types_proc_macros::HasMacro;
use ordered_float::NotNan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub struct WorldCoordinate {
    pub relative: bool,
    pub value: Option<NotNan<f32>>,
}

impl WorldCoordinate {
    #[inline]
    #[must_use]
    pub fn new(relative: bool, value: Option<NotNan<f32>>) -> Self {
        if !relative && value.is_none() {
            panic!("A world coordinate must have a relative coordinate and/or have a value");
        }

        Self { relative, value }
    }

    #[inline]
    #[must_use]
    pub fn relative(value: NotNan<f32>) -> Self {
        Self::new(true, Some(value))
    }

    #[inline]
    #[must_use]
    pub fn relative_optional(value: Option<NotNan<f32>>) -> Self {
        Self::new(true, value)
    }

    #[inline]
    #[must_use]
    pub fn absolute(value: NotNan<f32>) -> Self {
        Self::new(false, Some(value))
    }

    #[inline]
    #[must_use]
    pub fn absolute_optional(value: Option<NotNan<f32>>) -> Self {
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
        Self::absolute(NotNan::new(0.0).unwrap())
    }
}

impl Display for WorldCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.relative {
            "~".fmt(f)?;
        }

        if let Some(value) = self.value {
            value.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, HasMacro)]
pub enum Coordinates {
    World(WorldCoordinate, WorldCoordinate, WorldCoordinate),
    Local(
        Option<NotNan<f32>>,
        Option<NotNan<f32>>,
        Option<NotNan<f32>>,
    ),
}

impl Coordinates {
    #[inline]
    #[must_use]
    pub fn new_world(x: WorldCoordinate, y: WorldCoordinate, z: WorldCoordinate) -> Self {
        Coordinates::World(x, y, z)
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

    #[inline]
    #[must_use]
    pub fn new_local(
        x: Option<NotNan<f32>>,
        y: Option<NotNan<f32>>,
        z: Option<NotNan<f32>>,
    ) -> Self {
        Coordinates::Local(x, y, z)
    }

    #[inline]
    #[must_use]
    pub fn new_local_zero() -> Self {
        Self::new_local(None, None, None)
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Coordinates::World(x, y, z) => {
                write!(f, "{} {} {}", x, y, z)
            }
            Coordinates::Local(x, y, z) => {
                "^".fmt(f)?;

                if let Some(x) = x {
                    x.fmt(f)?;
                }

                " ^".fmt(f)?;

                if let Some(y) = y {
                    y.fmt(f)?;
                }

                " ^".fmt(f)?;

                if let Some(z) = z {
                    z.fmt(f)?;
                }

                Ok(())
            }
        }
    }
}
