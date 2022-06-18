use std::{
    io,
    process::{Command as StdCommand, ExitStatus as StdExitStatus, Output as StdOutput, Stdio},
};

pub struct Command<'a> {
    program: &'a str,
    args: Vec<&'a str>,
    execute_fn: ExecuteFn,
}

impl<'a> Command<'a> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            args: vec![],
            execute_fn: Box::new(|program, args| {
                let output = StdCommand::new(program)
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()?;
                Ok(Box::new(output))
            }),
        }
    }

    pub fn args(&self) -> &[&str] {
        &self.args
    }

    pub fn execute(&self) -> io::Result<Box<dyn Output>> {
        (self.execute_fn)(self.program, &self.args)
    }

    pub fn program(&self) -> &str {
        self.program
    }

    pub fn with_args(mut self, args: Vec<&'a str>) -> Self {
        self.args = args;
        self
    }
}

pub trait ExitStatus {
    fn code(&self) -> Option<i32>;

    fn success(&self) -> bool;
}

impl ExitStatus for StdExitStatus {
    fn code(&self) -> Option<i32> {
        self.code()
    }

    fn success(&self) -> bool {
        self.success()
    }
}

pub trait Output {
    fn status(&self) -> &dyn ExitStatus;

    fn stderr(&self) -> String;

    fn stdout(&self) -> String;
}

impl Output for StdOutput {
    fn status(&self) -> &dyn ExitStatus {
        &self.status
    }

    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }

    fn stdout(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
}

type ExecuteFn = Box<dyn Fn(&str, &[&str]) -> io::Result<Box<dyn Output>>>;

#[cfg(test)]
pub mod test {
    use super::*;

    #[derive(Default)]
    pub struct ExitStatusStub(i32);

    impl ExitStatus for ExitStatusStub {
        fn code(&self) -> Option<i32> {
            Some(self.0)
        }

        fn success(&self) -> bool {
            self.0 == 0
        }
    }

    #[derive(Default)]
    pub struct OutputStub {
        exit_status: ExitStatusStub,
        stdout: String,
        stderr: String,
    }

    impl OutputStub {
        pub fn new(code: i32, stdout: String, stderr: String) -> Self {
            Self {
                exit_status: ExitStatusStub(code),
                stdout,
                stderr,
            }
        }
    }

    impl Output for OutputStub {
        fn status(&self) -> &dyn ExitStatus {
            &self.exit_status
        }

        fn stderr(&self) -> String {
            self.stderr.clone()
        }

        fn stdout(&self) -> String {
            self.stdout.clone()
        }
    }

    mod command {
        use super::*;

        mod new {
            use super::*;

            #[test]
            fn should_return_command() {
                let program = "echo";
                let cmd = Command::new(program);
                assert_eq!(cmd.program, program);
            }
        }

        mod args {
            use super::*;

            #[test]
            fn should_return_args() {
                let expected = vec!["-n", "it works!"];
                let cmd = Command {
                    program: "echo",
                    args: expected.clone(),
                    execute_fn: Box::new(move |_, _| Ok(Box::new(OutputStub::default()))),
                };
                assert_eq!(cmd.args(), expected);
            }
        }

        mod execute {
            use super::*;

            #[test]
            fn should_return_err() {
                let expected = io::ErrorKind::PermissionDenied;
                let cmd = Command {
                    program: "echo",
                    args: vec![],
                    execute_fn: Box::new(move |_, _| Err(io::Error::from(expected))),
                };
                match cmd.execute() {
                    Ok(_) => panic!("should be fail"),
                    Err(err) => assert_eq!(err.kind(), expected),
                }
            }

            #[test]
            fn should_return_output() {
                let code = 0;
                let stdout = String::from("stdout");
                let stderr = String::from("stderr");
                let cmd = Command {
                    program: "echo",
                    args: vec![],
                    execute_fn: Box::new({
                        let stdout = stdout.clone();
                        let stderr = stderr.clone();
                        move |_, _| {
                            Ok(Box::new(OutputStub::new(
                                code,
                                stdout.clone(),
                                stderr.clone(),
                            )))
                        }
                    }),
                };
                let output = cmd.execute().unwrap();
                assert_eq!(output.status().code(), Some(code));
                assert_eq!(output.stdout(), stdout);
                assert_eq!(output.stderr(), stderr);
            }
        }

        mod program {
            use super::*;

            #[test]
            fn should_return_program() {
                let expected = "echo";
                let cmd = Command {
                    program: expected,
                    args: vec![],
                    execute_fn: Box::new(move |_, _| Ok(Box::new(OutputStub::default()))),
                };
                assert_eq!(cmd.program(), expected);
            }
        }

        mod with_args {
            use super::*;

            #[test]
            fn should_set_args() {
                let expected = vec!["-n", "it works!"];
                let cmd = Command {
                    program: "echo",
                    args: vec![],
                    execute_fn: Box::new(|_, _| Ok(Box::new(OutputStub::default()))),
                };
                let cmd = cmd.with_args(expected.clone());
                assert_eq!(cmd.args, expected);
            }
        }
    }
}
