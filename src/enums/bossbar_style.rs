#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Display, EnumString)]
pub enum BossbarStyle {
    #[strum(serialize = "notched_6")]
    Notched6,
    #[strum(serialize = "notched_10")]
    Notched10,
    #[strum(serialize = "notched_12")]
    Notched12,
    #[strum(serialize = "notched_20")]
    Notched20,
    #[default]
    Progress,
}
