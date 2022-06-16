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
        let mut ctx = Context::new(self);
        for action in self.actions.iter() {
            let action_name = action.name();
            info!("[{}] Executing action '{}'", self.name, action_name);
            let output = action.run(&ctx);
            info!(
                "[{}] Action '{}' terminated with status: {}",
                self.name,
                action_name,
                output.status()
            );
            ctx.update(action_name, output);
        }
        ctx.take_outputs()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::*;

    mod workflow {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn should_return_workflow() {
                let name = "workflow1";
                let workflow = Workflow::new(String::from(name));
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
                    name: String::from(expected),
                    actions: vec![],
                };
                assert_eq!(workflow.name(), expected);
            }
        }

        mod run {
            use super::*;
            use crate::{test::*, var::*};
            use simplelog::{Config, LevelFilter, WriteLogger};

            #[test]
            fn should_return_outputs() {
                let workflow_name = "workflow1";
                let action1_name = "action1";
                let action1_status = Status::Unchanged;
                let action1_output = Output::new(action1_status).add_var("foo1", Var::Integer(15));
                let action2_name = "action2";
                let action2_status = Status::Changed;
                let action2_output =
                    Output::new(action2_status).add_var("foo2", Var::Boolean(true));
                let action3_name = "action3";
                let action3_status = Status::Failed;
                let action3_output =
                    Output::new(action3_status).add_var("foo3", Var::String(String::from("bar1")));
                let action4_name = "action4";
                let action4_status = Status::Skipped;
                let action4_output = Output::new(action4_status).add_var(
                    "foo4",
                    Var::Hash(vars!(String::from("foo4"), Var::Integer(15))),
                );
                let expected = outputs!(
                    action1_name,
                    action1_output.clone(),
                    action2_name,
                    action2_output.clone(),
                    action3_name,
                    action3_output.clone(),
                    action4_name,
                    action4_output.clone()
                );
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
                        let expected = outputs!(action1_name, action1_output.clone());
                        assert_eq!(ctx.outputs().clone(), expected);
                        action2_output.clone()
                    }
                });
                let action3 = action_stub!(action3_name, {
                    let action1_output = action1_output.clone();
                    let action2_output = action2_output.clone();
                    let action3_output = action3_output.clone();
                    move |ctx| {
                        let expected = outputs!(
                            action1_name,
                            action1_output.clone(),
                            action2_name,
                            action2_output.clone()
                        );
                        assert_eq!(ctx.outputs().clone(), expected);
                        action3_output.clone()
                    }
                });
                let action4 = action_stub!(action4_name, move |ctx| {
                    let expected = outputs!(
                        action1_name,
                        action1_output.clone(),
                        action2_name,
                        action2_output.clone(),
                        action3_name,
                        action3_output.clone()
                    );
                    assert_eq!(ctx.outputs().clone(), expected);
                    action4_output.clone()
                });
                let workflow = Workflow {
                    name: String::from(workflow_name),
                    actions: vec![action1, action2, action3, action4],
                };
                let logs = vec![
                    format!(
                        "[INFO] [{}] Executing action '{}'",
                        workflow_name, action1_name
                    ),
                    format!(
                        "[INFO] [{}] Action '{}' terminated with status: {}",
                        workflow_name, action1_name, action1_status
                    ),
                    format!(
                        "[INFO] [{}] Executing action '{}'",
                        workflow_name, action2_name
                    ),
                    format!(
                        "[INFO] [{}] Action '{}' terminated with status: {}",
                        workflow_name, action2_name, action2_status
                    ),
                    format!(
                        "[INFO] [{}] Executing action '{}'",
                        workflow_name, action3_name
                    ),
                    format!(
                        "[INFO] [{}] Action '{}' terminated with status: {}",
                        workflow_name, action3_name, action3_status
                    ),
                    format!(
                        "[INFO] [{}] Executing action '{}'",
                        workflow_name, action4_name
                    ),
                    format!(
                        "[INFO] [{}] Action '{}' terminated with status: {}",
                        workflow_name, action4_name, action4_status
                    ),
                ];
                let log_asserter = LogAsserter::new(logs);
                WriteLogger::init(LevelFilter::Trace, Config::default(), log_asserter).unwrap();
                let outputs = workflow.run();
                assert_eq!(outputs, expected);
            }
        }
    }
}
