pub mod bash;

use crate::{context::*, var::*};
use std::fmt::{self, Display, Formatter};

pub static ACTION_NAME_PATTERN: &str = "[A-z0-9_]+";

pub trait Action {
    fn name(&self) -> &str;

    fn run(&self, ctx: &Context) -> Output;
}

pub trait Builder {
    fn build(self, ctx: &Context) -> Result<Box<dyn Action>, BuildError>;
}

#[derive(Debug)]
pub enum BuildError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Output {
    status: Status,
    vars: Hash,
}

impl Output {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            vars: Hash::new(),
        }
    }

    pub fn add_var(mut self, name: &str, val: Value) -> Self {
        self.vars.insert(name.into(), val);
        self
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn value(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn vars(&self) -> &Hash {
        &self.vars
    }

    pub fn with_vars(mut self, vars: Hash) -> Self {
        self.vars = vars;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Unchanged,
    Changed,
    Failed,
    Skipped,
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
    use crate::*;

    macro_rules! action_stub {
        ($name:expr, $run_fn:expr) => {
            Box::new(ActionStub::new($name, Box::new($run_fn)))
        };
    }

    pub(crate) use action_stub;

    pub struct ActionStub {
        name: &'static str,
        run_fn: RunFn,
    }

    impl ActionStub {
        pub fn new(name: &'static str, run_fn: RunFn) -> Self {
            Self { name, run_fn }
        }
    }

    impl Action for ActionStub {
        fn name(&self) -> &str {
            self.name
        }

        fn run(&self, ctx: &Context) -> Output {
            (self.run_fn)(ctx)
        }
    }

    pub type RunFn = Box<dyn Fn(&Context) -> Output>;

    mod output {
        use super::*;

        mod add_var {
            use super::*;

            #[test]
            fn should_add_var() {
                let name = "foo";
                let val = 15u8;
                let expected = hash!(name, val);
                let output = Output {
                    status: Status::Changed,
                    vars: Hash::new(),
                };
                let output = output.add_var(name, val.into());
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
                    vars: Hash::new(),
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
                    vars: Hash::new(),
                };
                assert_eq!(output.status(), expected);
            }
        }

        mod value {
            use super::*;

            #[test]
            fn should_return_none() {
                let output = Output {
                    status: Status::Changed,
                    vars: Hash::new(),
                };
                let val = output.value("foo");
                assert!(val.is_none());
            }

            #[test]
            fn should_return_val() {
                let name = "foo";
                let expected = 15u8;
                let output = Output {
                    status: Status::Changed,
                    vars: hash!(name, expected),
                };
                let val = output.value(name).unwrap();
                assert_eq!(*val, expected.into());
            }
        }

        mod vars {
            use super::*;

            #[test]
            fn should_return_vars() {
                let expected = hash!(String::from("foo"), Value::from(15u8));
                let output = Output {
                    status: Status::Changed,
                    vars: expected.clone(),
                };
                let vars = output.vars();
                assert_eq!(*vars, expected);
            }
        }

        mod with_vars {
            use super::*;

            #[test]
            fn should_set_vars() {
                let expected = hash!(String::from("foo"), Value::from(15u8));
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
