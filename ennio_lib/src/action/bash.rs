use crate::{
    action::*,
    command::{Command, Output as CmdOutput},
};
use log::{debug, error};
use std::io;

pub struct BashAction {
    name: String,
    script: String,
    execute_fn: ExecuteFn,
}

impl BashAction {
    pub fn new(name: String, script: String) -> Self {
        Self {
            name,
            script,
            execute_fn: Box::new(|cmd| cmd.execute()),
        }
    }
}

impl Action for BashAction {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self, _ctx: &Context) -> Output {
        let cmd = Command::new("bash").with_args(vec!["-ec", &self.script]);
        match (self.execute_fn)(&cmd) {
            Ok(output) => {
                let stderr = output.stderr();
                let status = if output.status().success() {
                    debug!("Script executed successfully");
                    Status::Changed
                } else {
                    debug!("Script execution failed:\n{}", stderr);
                    Status::Failed
                };
                Output::new(status)
                    .add_var("stdout", Var::String(output.stdout()))
                    .add_var("stderr", Var::String(stderr))
            }
            Err(err) => {
                error!("Unable to execute script: {}", err);
                Output::new(Status::Failed).add_var("stderr", Var::String(err.to_string()))
            }
        }
    }
}

type ExecuteFn = Box<dyn Fn(&Command) -> io::Result<Box<dyn CmdOutput>>>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{command::test::*, test::*};

    mod bash_action {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn should_return_action() {
                let name = "action1";
                let script = "echo 'it works!'";
                let action = BashAction::new(name.into(), script.into());
                assert_eq!(action.name, name);
                assert_eq!(action.script, script);
            }
        }

        mod name {
            use super::*;

            #[test]
            fn should_return_name() {
                let name = "action1";
                let action = BashAction {
                    name: name.into(),
                    script: String::from("echo 'it works!'"),
                    execute_fn: Box::new(|cmd| cmd.execute()),
                };
                assert_eq!(action.name(), name);
            }
        }

        mod run {
            use super::*;

            macro_rules! test {
                ($code:expr, $status:expr, $create_log_fn:expr) => {
                    let stdout = "stdout";
                    let stderr = "stderr";
                    let expected = Output::new($status)
                        .add_var("stdout", Var::String(stdout.into()))
                        .add_var("stderr", Var::String(stderr.into()));
                    let ctx = Context::new("workflow1");
                    let script = "echo 'it works!'";
                    let action = BashAction {
                        name: String::from("action1"),
                        script: script.into(),
                        execute_fn: Box::new(move |cmd| {
                            assert_eq!(cmd.program(), "bash");
                            assert_eq!(cmd.args(), vec!["-ec", &script]);
                            Ok(Box::new(OutputStub::new(
                                $code,
                                stdout.into(),
                                stderr.into(),
                            )))
                        }),
                    };
                    init_logger();
                    unsafe {
                        let logger = LOGGER.as_mut().unwrap();
                        logger.expect_logs(vec![$create_log_fn(stderr)]);
                    }
                    let output = action.run(&ctx);
                    assert_eq!(output, expected);
                };
            }

            #[test]
            fn should_return_output_with_failed_status_if_io_err() {
                let err_kind = io::ErrorKind::PermissionDenied;
                let expected = Output::new(Status::Failed)
                    .add_var("stderr", Var::String(io::Error::from(err_kind).to_string()));
                let ctx = Context::new("workflow1");
                let script = "echo 'it works!'";
                let action = BashAction {
                    name: String::from("action1"),
                    script: script.into(),
                    execute_fn: Box::new(move |cmd| {
                        assert_eq!(cmd.program(), "bash");
                        assert_eq!(cmd.args(), vec!["-ec", script]);
                        Err(io::Error::from(err_kind))
                    }),
                };
                init_logger();
                unsafe {
                    let logger = LOGGER.as_mut().unwrap();
                    logger.expect_logs(vec![format!(
                        "Unable to execute script: {}",
                        io::Error::from(err_kind)
                    )]);
                }
                let output = action.run(&ctx);
                assert_eq!(output, expected);
            }

            #[test]
            fn should_return_output_with_failed_status_if_exit_status_is_not_success() {
                test!(1, Status::Failed, |stderr| format!(
                    "Script execution failed:\n{}",
                    stderr
                ));
            }

            #[test]
            fn should_return_output_with_changed_status() {
                test!(0, Status::Changed, |_| String::from(
                    "Script executed successfully"
                ));
            }
        }
    }
}
