use crate::{action::*, context::*};
use log::info;

pub struct Workflow {
    name: String,
    actions: Vec<Box<dyn Action>>,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            name,
            actions: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn run(&self) -> Outputs {
        let mut ctx = Context::new(&self.name);
        for action in self.actions.iter() {
            let action_name = action.name();
            info!("Executing action '{}'", action_name);
            let output = action.run(&ctx);
            info!(
                "Action '{}' terminated with status: {}",
                action_name,
                output.status()
            );
            ctx.update(action_name, output);
        }
        ctx.take_outputs()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{action::test::*, var::*, *};

    mod workflow {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn should_return_workflow() {
                let name = "workflow1";
                let workflow = Workflow::new(name.into());
                assert_eq!(workflow.name, name);
                assert!(workflow.actions.is_empty());
            }
        }

        mod name {
            use super::*;

            #[test]
            fn should_return_name() {
                let expected = "workflow1";
                let workflow = Workflow {
                    name: expected.into(),
                    actions: vec![],
                };
                assert_eq!(workflow.name(), expected);
            }
        }

        mod run {
            use super::*;

            #[test]
            fn should_return_outputs() {
                let workflow_name = "workflow1";
                let action1_name = "action1";
                let action1_status = Status::Unchanged;
                let action1_output = Output::new(action1_status).add_var("foo1", Value::from(15u8));
                let action2_name = "action2";
                let action2_status = Status::Changed;
                let action2_output = Output::new(action2_status).add_var("foo2", Value::Bool(true));
                let action3_name = "action3";
                let action3_status = Status::Failed;
                let action3_output = Output::new(action3_status)
                    .add_var("foo3", Value::String(String::from("bar1")));
                let action4_name = "action4";
                let action4_status = Status::Skipped;
                let action4_output =
                    Output::new(action4_status).add_var("foo4", Value::Hash(hash!("foo4", 15u8)));
                let expected = Outputs::from([
                    (action1_name.into(), action1_output.clone()),
                    (action2_name.into(), action2_output.clone()),
                    (action3_name.into(), action3_output.clone()),
                    (action4_name.into(), action4_output.clone()),
                ]);
                let action1 = action_stub!(action1_name, {
                    let action1_output = action1_output.clone();
                    move |ctx| {
                        assert!(ctx.outputs().is_empty());
                        action1_output.clone()
                    }
                });
                let action2 = action_stub!(action2_name, {
                    let action1_output = action1_output.clone();
                    let action2_output = action2_output.clone();
                    move |ctx| {
                        let expected =
                            Outputs::from([(action1_name.into(), action1_output.clone())]);
                        assert_eq!(ctx.outputs().clone(), expected);
                        action2_output.clone()
                    }
                });
                let action3 = action_stub!(action3_name, {
                    let action1_output = action1_output.clone();
                    let action2_output = action2_output.clone();
                    let action3_output = action3_output.clone();
                    move |ctx| {
                        let expected = Outputs::from([
                            (action1_name.into(), action1_output.clone()),
                            (action2_name.into(), action2_output.clone()),
                        ]);
                        assert_eq!(ctx.outputs().clone(), expected);
                        action3_output.clone()
                    }
                });
                let action4 = action_stub!(action4_name, move |ctx| {
                    let expected = Outputs::from([
                        (action1_name.into(), action1_output.clone()),
                        (action2_name.into(), action2_output.clone()),
                        (action3_name.into(), action3_output.clone()),
                    ]);
                    assert_eq!(ctx.outputs().clone(), expected);
                    action4_output.clone()
                });
                let workflow = Workflow {
                    name: workflow_name.into(),
                    actions: vec![action1, action2, action3, action4],
                };
                let outputs = workflow.run();
                assert_eq!(outputs, expected);
            }
        }
    }
}
