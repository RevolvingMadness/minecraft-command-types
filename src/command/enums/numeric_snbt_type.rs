use crate::create_enum;

create_enum!(
    NumericSnbtType,
    [],
    Byte,
    Short,
    #[strum(serialize = "int")]
    Integer,
    Long,
    Float,
    Double
);
