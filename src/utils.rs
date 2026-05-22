#[macro_export]
macro_rules! option_write_chain {
    ($f:expr, $($option:expr),* $(,)?) => {
        $(
            if let Some(v) = $option {
                write!($f, " {}", v)?;
            } else {
                return Ok(());
            }
        )*
    };
}
