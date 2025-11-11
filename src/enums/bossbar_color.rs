#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum BossbarColor {
    Blue,
    Green,
    Pink,
    Purple,
    Red,
    #[default]
    White,
    Yellow
}
