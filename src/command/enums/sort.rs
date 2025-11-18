use crate::create_enum;

create_enum!(
    Sort,
    [Default],
    #[default]
    Arbitrary,
    Furthest,
    Nearest,
    Random
);
