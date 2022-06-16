pub mod action;
pub mod context;
pub mod var;

#[macro_export]
macro_rules! outputs {
    ($($name:expr, $output:expr),*) => {
        Outputs::from([$((String::from($name), $output)),*])
    };
}

#[macro_export]
macro_rules! vars {
    ($($name:expr, $value:expr),*) => {
        Vars::from([$((String::from($name), $value)),*])
    };
}
