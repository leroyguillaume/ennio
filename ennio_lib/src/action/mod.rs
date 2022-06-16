use crate::{context::*, var::*, *};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

pub trait Action {
    fn name(&self) -> &str;

    fn run(&self, ctx: &Context) -> Output;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Output {
    status: Status,
    vars: Vars,
}

pub type Outputs = HashMap<String, Output>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Unchanged,
    Changed,
    Failed,
    Skipped,
}

impl Output {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            vars: vars!(),
        }
    }

    pub fn add_var(mut self, name: &str, var: Var) -> Self {
        self.vars.insert(String::from(name), var);
        self
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn var(&self, name: &str) -> Option<Var> {
        self.vars.get(name).cloned()
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }

    pub fn with_vars(mut self, vars: Vars) -> Self {
        self.vars = vars;
        self
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let status = match self {
            Self::Unchanged => "unchanged",
            Self::Changed => "changed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        };
        write!(f, "{}", status)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    mod output {
        use super::*;

        mod add_var {
            use super::*;

            #[test]
            fn should_add_var() {
                let name = "foo";
                let var = Var::Integer(15);
                let expected = vars!(name, var.clone());
                let output = Output {
                    status: Status::Changed,
                    vars: Vars::new(),
                };
                let output = output.add_var(name, var);
                assert_eq!(output.vars, expected);
            }
        }

        mod new {
            use super::*;

            #[test]
            fn should_return_output() {
                let status = Status::Changed;
                let expected = Output {
                    status,
                    vars: vars!(),
                };
                let output = Output::new(status);
                assert_eq!(output, expected);
            }
        }

        mod status {
            use super::*;

            #[test]
            fn should_return_status() {
                let expected = Status::Changed;
                let output = Output {
                    status: expected,
                    vars: vars!(),
                };
                assert_eq!(output.status(), expected);
            }
        }

        mod vars {
            use super::*;

            #[test]
            fn should_return_vars() {
                let expected = vars!("foo", Var::Integer(15));
                let output = Output {
                    status: Status::Changed,
                    vars: expected.clone(),
                };
                let vars = output.vars();
                assert_eq!(*vars, expected);
            }
        }

        mod var {
            use super::*;

            #[test]
            fn should_return_none() {
                let output = Output {
                    status: Status::Changed,
                    vars: vars!(),
                };
                let var = output.var("foo");
                assert!(var.is_none());
            }

            #[test]
            fn should_return_var() {
                let name = "foo";
                let expected = Var::Integer(15);
                let output = Output {
                    status: Status::Changed,
                    vars: vars!(name, expected.clone()),
                };
                let var = output.var(name).unwrap();
                assert_eq!(var, expected);
            }
        }

        mod with_vars {
            use super::*;

            #[test]
            fn should_set_vars() {
                let expected = vars!("foo", Var::Integer(15));
                let output = Output {
                    status: Status::Changed,
                    vars: expected.clone(),
                };
                let output = output.with_vars(expected.clone());
                assert_eq!(output.vars, expected);
            }
        }
    }

    mod status {
        use super::*;

        mod display {
            use super::*;

            macro_rules! test {
                ($value:expr, $expected:ident) => {
                    mod $expected {
                        use super::*;

                        #[test]
                        fn test() {
                            assert_eq!(format!("{}", $value), stringify!($expected));
                        }
                    }
                };
            }

            test!(Status::Unchanged, unchanged);
            test!(Status::Changed, changed);
            test!(Status::Failed, failed);
            test!(Status::Skipped, skipped);
        }
    }
}
