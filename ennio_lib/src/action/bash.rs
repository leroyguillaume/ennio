use crate::{
    action::*,
    command::{Command, Output as CmdOutput},
};
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
                let status = if output.status().success() {
                    Status::Changed
                } else {
                    Status::Failed
                };
                Output::new(status)
                    .add_var("stdout", Var::String(output.stdout()))
                    .add_var("stderr", Var::String(output.stderr()))
            }
            Err(err) => Output::new(Status::Failed).add_var("stderr", Var::String(err.to_string())),
        }
    }
}

type ExecuteFn = Box<dyn Fn(&Command) -> io::Result<Box<dyn CmdOutput>>>;

#[cfg(test)]
mod test {
    use super::*;

    mod bash_action {
        use super::*;
        use crate::command::test::*;

        mod new {
            use super::*;

            #[test]
            fn should_return_action() {
                let name = String::from("action1");
                let script = String::from("echo 'it works!'");
                let action = BashAction::new(name.clone(), script.clone());
                assert_eq!(action.name, name);
                assert_eq!(action.script, script);
            }
        }

        mod name {
            use super::*;

            #[test]
            fn should_return_name() {
                let name = String::from("action1");
                let action = BashAction {
                    name: name.clone(),
                    script: String::from("echo 'it works!'"),
                    execute_fn: Box::new(|cmd| cmd.execute()),
                };
                assert_eq!(action.name(), name);
            }
        }

        mod run {
            use super::*;

            macro_rules! test {
                ($code:expr, $status:expr) => {
                    let stdout = "stdout";
                    let stderr = "stderr";
                    let expected = Output::new($status)
                        .add_var("stdout", Var::String(String::from(stdout)))
                        .add_var("stderr", Var::String(String::from(stderr)));
                    let ctx = Context::new("workflow1");
                    let script = String::from("echo 'it works!'");
                    let action = BashAction {
                        name: String::from("action1"),
                        script: script.clone(),
                        execute_fn: Box::new(move |cmd| {
                            assert_eq!(cmd.program(), "bash");
                            assert_eq!(cmd.args(), vec!["-ec", &script]);
                            Ok(Box::new(OutputStub::new(
                                $code,
                                String::from(stdout),
                                String::from(stderr),
                            )))
                        }),
                    };
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
                let script = String::from("echo 'it works!'");
                let action = BashAction {
                    name: String::from("action1"),
                    script: script.clone(),
                    execute_fn: Box::new(move |cmd| {
                        assert_eq!(cmd.program(), "bash");
                        assert_eq!(cmd.args(), vec!["-ec", &script]);
                        Err(io::Error::from(err_kind))
                    }),
                };
                let output = action.run(&ctx);
                assert_eq!(output, expected);
            }

            #[test]
            fn should_return_output_with_failed_status_if_exit_status_is_not_success() {
                test!(1, Status::Failed);
            }

            #[test]
            fn should_return_output_with_changed_status() {
                test!(0, Status::Changed);
            }
        }
    }
}
