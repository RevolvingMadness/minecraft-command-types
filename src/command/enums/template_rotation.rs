use crate::create_enum;

create_enum!(
    TemplateRotation,
    [],
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "clockwise_90")]
    Clockwise90,
    #[strum(serialize = "counterclockwise_90")]
    CounterClockwise90,
    #[strum(serialize = "180")]
    OneEighty
);
