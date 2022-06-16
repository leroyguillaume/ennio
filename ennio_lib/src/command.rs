use std::{
    io,
    process::{Command as StdCommand, ExitStatus as StdExitStatus, Output as StdOutput, Stdio},
};

pub struct Command {
    program: String,
    args: Vec<String>,
    execute_fn: ExecuteFn,
}

impl Command {
    pub fn new(program: String) -> Self {
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

    pub fn execute(&self) -> io::Result<Box<dyn Output>> {
        (self.execute_fn)(&self.program, &self.args)
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

pub type ExecuteFn = Box<dyn Fn(&str, &[String]) -> io::Result<Box<dyn Output>>>;

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

#[cfg(test)]
mod test {
    use super::*;

    pub struct ExitStatusStub(i32);

    impl ExitStatus for ExitStatusStub {
        fn code(&self) -> Option<i32> {
            Some(self.0)
        }

        fn success(&self) -> bool {
            self.0 == 0
        }
    }

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
                let program = String::from("echo");
                let cmd = Command::new(program.clone());
                assert_eq!(cmd.program, program);
            }
        }

        mod execute {
            use super::*;

            #[test]
            fn should_return_err() {
                let expected = io::ErrorKind::PermissionDenied;
                let cmd = Command {
                    program: String::from("echo"),
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
                    program: String::from("echo"),
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

        mod with_args {
            use super::*;

            #[test]
            fn should_set_args() {
                let expected = vec![String::from("-n"), String::from("it works!")];
                let cmd = Command {
                    program: String::from("echo"),
                    args: vec![],
                    execute_fn: Box::new(|_, _| {
                        Ok(Box::new(OutputStub::new(0, String::new(), String::new())))
                    }),
                };
                let cmd = cmd.with_args(expected.clone());
                assert_eq!(cmd.args, expected);
            }
        }
    }
}
