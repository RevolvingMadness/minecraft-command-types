use crate::create_enum;

create_enum!(
    BossbarColor,
    [Default],
    Blue,
    Green,
    Pink,
    Purple,
    Red,
    #[default]
    White,
    Yellow
);
