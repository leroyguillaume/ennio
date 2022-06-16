use crate::{action::*, var::*, workflow::*};

pub struct Context<'a> {
    workflow: &'a Workflow,
    outputs: Outputs,
}

impl<'a> Context<'a> {
    pub fn new(workflow: &'a Workflow) -> Self {
        Self {
            workflow,
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
        self.outputs.insert(String::from(name), output);
    }

    pub fn var(&self, action_name: &str, name: &str) -> Option<Var> {
        self.output(action_name).and_then(|output| output.var(name))
    }

    pub fn workflow(&self) -> &Workflow {
        self.workflow
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::*;

    mod new {
        use super::*;

        #[test]
        fn should_return_context() {
            let workflow = Workflow::new("workflow1");
            let ctx = Context::new(&workflow);
            assert_eq!(ctx.workflow.name(), workflow.name());
            assert!(ctx.outputs.is_empty());
        }
    }

    mod output {
        use super::*;

        #[test]
        fn should_return_none() {
            let workflow = Workflow::new("workflow1");
            let ctx = Context {
                workflow: &workflow,
                outputs: outputs!(),
            };
            let output = ctx.output("action1");
            assert!(output.is_none());
        }

        #[test]
        fn should_return_output() {
            let workflow = Workflow::new("workflow1");
            let name = "action1";
            let expected = Output::new(Status::Changed);
            let ctx = Context {
                workflow: &workflow,
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
            let workflow = Workflow::new("workflow1");
            let output = Output::new(Status::Changed);
            let expected = outputs!("action1", output);
            let ctx = Context {
                workflow: &workflow,
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
            let workflow = Workflow::new("workflow1");
            let output = Output::new(Status::Changed);
            let expected = outputs!("action1", output);
            let ctx = Context {
                workflow: &workflow,
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
            let workflow = Workflow::new("workflow1");
            let name = "action1";
            let output = Output::new(Status::Changed);
            let expected = outputs!(name, output.clone());
            let mut ctx = Context {
                workflow: &workflow,
                outputs: outputs!(),
            };
            ctx.update(name, output);
            assert_eq!(ctx.outputs, expected);
        }
    }

    mod var {
        use super::*;

        #[test]
        fn should_return_none_if_no_outputs() {
            let workflow = Workflow::new("workflow1");
            let ctx = Context {
                workflow: &workflow,
                outputs: outputs!(),
            };
            let var = ctx.var("action1", "foo");
            assert!(var.is_none());
        }

        #[test]
        fn should_return_none_if_no_vars_in_ouputs() {
            let output_name = "action1";
            let workflow = Workflow::new("workflow1");
            let output = Output::new(Status::Changed);
            let ctx = Context {
                workflow: &workflow,
                outputs: outputs!(output_name, output),
            };
            let var = ctx.var(output_name, "foo");
            assert!(var.is_none());
        }

        #[test]
        fn should_return_var() {
            let output_name = "action1";
            let name = "foo";
            let expected = Var::Integer(15);
            let workflow = Workflow::new("workflow1");
            let output = Output::new(Status::Changed).add_var(name, expected.clone());
            let ctx = Context {
                workflow: &workflow,
                outputs: outputs!(output_name, output),
            };
            let var = ctx.var(output_name, name).unwrap();
            assert_eq!(var, expected);
        }
    }

    mod workflow {
        use super::*;

        #[test]
        fn should_return_workflow() {
            let expected = Workflow::new("workflow1");
            let ctx = Context {
                workflow: &expected,
                outputs: outputs!(),
            };
            let workflow = ctx.workflow();
            assert_eq!(workflow.name(), expected.name());
        }
    }
}
