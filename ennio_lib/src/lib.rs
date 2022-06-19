pub mod action;
pub mod command;
pub mod context;
pub mod workflow;

#[macro_export]
macro_rules! outputs {
    ($($name:expr, $output:expr),*) => {
        Outputs::from([$((String::from($name), $output)),*])
    };
}

#[macro_export]
macro_rules! vars {
    ($($name:expr, $value:expr),*) => {
        Map::from_iter([$((String::from($name), $value)),*])
    };
}
