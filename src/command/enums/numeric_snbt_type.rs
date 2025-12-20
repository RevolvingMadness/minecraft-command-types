use crate::create_enum;

create_enum!(
    NumericSNBTType,
    [],
    Byte,
    Short,
    #[strum(serialize = "int")]
    Integer,
    Long,
    Float,
    Double
);
