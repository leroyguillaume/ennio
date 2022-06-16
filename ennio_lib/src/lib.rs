pub mod var;

#[macro_export]
macro_rules! vars {
    ($($name:expr, $value:expr),*) => {
        Vars::from([$((String::from($name), $value)),*])
    };
}
