#[macro_export]
macro_rules! unwrap_or_else {
    ($expr:expr, $fallback:block) => {
        match $expr {
            Some(x) => x,
            None => $fallback,
        }
    };
}
