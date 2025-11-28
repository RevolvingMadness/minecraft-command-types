use crate::create_enum;

create_enum!(
    ScoreOperationOperator,
    [],
    #[strum(serialize = "=")]
    Set,
    #[strum(serialize = "+=")]
    Add,
    #[strum(serialize = "-=")]
    Subtract,
    #[strum(serialize = "*=")]
    Multiply,
    #[strum(serialize = "/=")]
    Divide,
    #[strum(serialize = "%=")]
    Modulo,
    #[strum(serialize = "><")]
    Swap,
    #[strum(serialize = "<")]
    ChooseMinimum,
    #[strum(serialize = ">")]
    ChooseMaximum,
);
