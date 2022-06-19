use crate::{action::*, var::*};
use regex::Regex;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Context<'a> {
    workflow_name: &'a str,
    outputs: Outputs,
}

impl<'a> Context<'a> {
    pub fn new(workflow_name: &'a str) -> Self {
        Self {
            workflow_name,
            outputs: Outputs::new(),
        }
    }

    pub fn output(&self, action_name: &str) -> Option<&Output> {
        self.outputs.get(action_name)
    }

    pub fn outputs(&self) -> &Outputs {
        &self.outputs
    }

    pub fn take_outputs(self) -> Outputs {
        self.outputs
    }

    pub fn update(&mut self, name: &str, output: Output) {
        self.outputs.insert(name.into(), output);
    }

    pub fn value(&self, var_name: &str) -> Result<&Value, VarError> {
        let re = Regex::new(ACTION_NAME_PATTERN).unwrap();
        match re.find(var_name) {
            Some(mat) => {
                let action_name = mat.as_str();
                let output = self
                    .output(action_name)
                    .ok_or_else(|| VarError::UnknownAction(action_name.into()))?;
                let var_name = &var_name[action_name.len()..];
                if var_name.is_empty() {
                    Err(VarError::MissingVarName)
                } else {
                    let var_name = &var_name[1..];
                    output
                        .value(var_name)
                        .ok_or_else(|| VarError::UnknownVar(action_name.into(), var_name.into()))
                }
            }
            None => Err(VarError::InvalidSyntax(var_name.into())),
        }
    }

    pub fn workflow_name(&self) -> &str {
        self.workflow_name
    }
}

#[derive(Debug)]
pub enum VarError {
    InvalidSyntax(String),
    UnknownAction(String),
    MissingVarName,
    UnknownVar(String, String),
}

impl Display for VarError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::InvalidSyntax(var_name) => format!("'{}' is not a valid variable name", var_name),
            Self::UnknownAction(action_name) => format!("Action '{}' does not exist", action_name),
            Self::MissingVarName => String::from("Missing variable name"),
            Self::UnknownVar(action_name, var_name) => {
                format!("No variable '{}' in '{}' outputs", var_name, action_name)
            }
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    mod new {
        use super::*;

        #[test]
        fn should_return_context() {
            let workflow_name = "workflow1";
            let exepcted = Context {
                workflow_name,
                outputs: outputs!(),
            };
            let ctx = Context::new(workflow_name);
            assert_eq!(ctx, exepcted);
        }
    }

    mod output {
        use super::*;

        #[test]
        fn should_return_none() {
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(),
            };
            let output = ctx.output("action1");
            assert!(output.is_none());
        }

        #[test]
        fn should_return_output() {
            let name = "action1";
            let expected = Output::new(Status::Changed);
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(name, expected.clone()),
            };
            let output = ctx.output(name).unwrap();
            assert_eq!(*output, expected);
        }
    }

    mod outputs {
        use super::*;

        #[test]
        fn should_return_outputs() {
            let output = Output::new(Status::Changed);
            let expected = outputs!("action1", output);
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: expected.clone(),
            };
            let outputs = ctx.outputs();
            assert_eq!(*outputs, expected);
        }
    }

    mod take_outputs {
        use super::*;

        #[test]
        fn should_return_outputs() {
            let output = Output::new(Status::Changed);
            let expected = outputs!("action1", output);
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: expected.clone(),
            };
            let outputs = ctx.take_outputs();
            assert_eq!(outputs, expected);
        }
    }

    mod update {
        use super::*;

        #[test]
        fn should_update_outputs() {
            let name = "action1";
            let output = Output::new(Status::Changed);
            let expected = outputs!(name, output.clone());
            let mut ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(),
            };
            ctx.update(name, output);
            assert_eq!(ctx.outputs, expected);
        }
    }

    mod value {
        use super::*;

        #[test]
        fn should_return_invalid_syntex_if_wrong_syntex() {
            let expected = "éè";
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(),
            };
            match ctx.value(expected) {
                Ok(_) => panic!("should fail"),
                Err(VarError::InvalidSyntax(var_name)) => assert_eq!(var_name, expected),
                Err(err) => panic!("{}", err),
            }
        }

        #[test]
        fn should_return_unknown_action() {
            let expected = "foo";
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(),
            };
            match ctx.value(expected) {
                Ok(_) => panic!("should fail"),
                Err(VarError::UnknownAction(action_name)) => {
                    assert_eq!(action_name, expected)
                }
                Err(err) => panic!("{}", err),
            }
        }

        #[test]
        fn should_return_missing_var_name() {
            let action_name = "action1";
            let output = Output::new(Status::Changed);
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(action_name, output),
            };
            match ctx.value(action_name) {
                Ok(_) => panic!("should fail"),
                Err(VarError::MissingVarName) => {}
                Err(err) => panic!("{}", err),
            }
        }

        #[test]
        fn should_return_unknown_var() {
            let expected_action_name = "action1";
            let output = Output::new(Status::Changed);
            let expected_var_name = "foo";
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(expected_action_name, output),
            };
            match ctx.value(&format!("{}.{}", expected_action_name, expected_var_name)) {
                Ok(_) => panic!("should fail"),
                Err(VarError::UnknownVar(action_name, var_name)) => {
                    assert_eq!(action_name, expected_action_name);
                    assert_eq!(var_name, expected_var_name);
                }
                Err(err) => panic!("{}", err),
            }
        }

        #[test]
        fn should_return_var() {
            let action_name = "action1";
            let var_name = "foo";
            let expected = Value::Bool(true);
            let output = Output::new(Status::Changed).add_var(var_name, expected.clone());
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(action_name, output),
            };
            let val = ctx.value(&format!("{}.{}", action_name, var_name)).unwrap();
            assert_eq!(val.clone(), expected);
        }
    }

    mod workflow_name {
        use super::*;

        #[test]
        fn should_return_workflow() {
            let expected = "workflow1";
            let ctx = Context {
                workflow_name: expected,
                outputs: outputs!(),
            };
            assert_eq!(ctx.workflow_name(), expected);
        }
    }

    mod var_error {
        use super::*;

        mod display {
            use super::*;

            macro_rules! test {
                ($name:ident, $value:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        assert_eq!(format!("{}", $value), $expected);
                    }
                };
            }

            test!(
                invalid_syntax,
                VarError::InvalidSyntax(String::from("éè")),
                "'éè' is not a valid variable name"
            );
            test!(
                unknown_action,
                VarError::UnknownAction(String::from("action1")),
                "Action 'action1' does not exist"
            );
            test!(
                missing_var_name,
                VarError::MissingVarName,
                "Missing variable name"
            );
            test!(
                unknown_var,
                VarError::UnknownVar(String::from("action1"), String::from("foo")),
                "No variable 'foo' in 'action1' outputs"
            );
        }
    }
}
