use crate::create_enum;

create_enum!(
    "camelCase",
    ScoreboardCriterion,
    [],
    Dummy,
    Trigger,
    DeathCount,
    PlayerKillCount,
    TotalKillCount,
    Health,
    XP,
    Level,
    Food,
    Air,
    Armor
);

impl ScoreboardCriterion {
    pub fn can_be_modified(&self) -> bool {
        match self {
            ScoreboardCriterion::Dummy
            | ScoreboardCriterion::Trigger
            | ScoreboardCriterion::DeathCount
            | ScoreboardCriterion::PlayerKillCount
            | ScoreboardCriterion::TotalKillCount => true,
            _ => false,
        }
    }
}
