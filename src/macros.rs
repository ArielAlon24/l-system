#[macro_export]
macro_rules! state {
    ($string:expr) => {{
        $string
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.into())
            .collect()
    }};
}

#[macro_export]
macro_rules! symbol {
    ($char:expr) => {
        Symbol::from($char)
    };
}
