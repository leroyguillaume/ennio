use crate::action::*;
use serde_json::Value;

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

    pub fn value(&self, action_name: &str, var_name: &str) -> Option<&Value> {
        self.output(action_name)
            .and_then(|output| output.value(var_name))
    }

    pub fn workflow_name(&self) -> &str {
        self.workflow_name
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use serde_json::value::Number;

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
        fn should_return_none_if_no_outputs() {
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(),
            };
            let val = ctx.value("action1", "foo");
            assert!(val.is_none());
        }

        #[test]
        fn should_return_none_if_no_vars_in_ouputs() {
            let output_name = "action1";
            let output = Output::new(Status::Changed);
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(output_name, output),
            };
            let val = ctx.value(output_name, "foo");
            assert!(val.is_none());
        }

        #[test]
        fn should_return_var() {
            let output_name = "action1";
            let name = "foo";
            let expected = Value::Number(Number::from(15i8));
            let output = Output::new(Status::Changed).add_var(name, expected.clone());
            let ctx = Context {
                workflow_name: "workflow1",
                outputs: outputs!(output_name, output),
            };
            let value = ctx.value(output_name, name).unwrap();
            assert_eq!(*value, expected);
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
}
