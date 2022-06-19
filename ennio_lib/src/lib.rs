pub mod action;
pub mod command;
pub mod config;
pub mod context;
pub mod var;
pub mod workflow;

#[macro_export]
macro_rules! outputs {
    ($($name:expr, $output:expr),*) => {
        Outputs::from([$((String::from($name), $output)),*])
    };
}
