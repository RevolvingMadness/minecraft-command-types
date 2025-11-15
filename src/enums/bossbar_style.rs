use crate::create_enum;

create_enum!(
    BossbarStyle,
    [Default],
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
);
