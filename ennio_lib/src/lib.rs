pub mod action;
pub mod command;
pub mod context;
pub mod var;
pub mod workflow;

#[macro_export]
macro_rules! outputs {
    ($($name:expr, $output:expr),*) => {
        Outputs::from([$((String::from($name), $output)),*])
    };
}

#[macro_export]
macro_rules! vars {
    ($($name:expr, $value:expr),*) => {
        Vars::from([$((String::from($name), $value)),*])
    };
}

#[cfg(test)]
mod test {
    use crate::{
        action::*,
        command::{ExitStatus, Output as CommandOutput},
        context::*,
    };
    use std::io::{self, Write};

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

    pub struct ExitStatusStub(i32);

    impl ExitStatus for ExitStatusStub {
        fn code(&self) -> Option<i32> {
            Some(self.0)
        }

        fn success(&self) -> bool {
            self.0 == 0
        }
    }

    pub struct LogAsserter {
        expected: Vec<String>,
        line: usize,
        buf: Vec<u8>,
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

    impl CommandOutput for OutputStub {
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

    pub type RunFn = Box<dyn Fn(&Context) -> Output>;

    impl LogAsserter {
        pub fn new(logs: Vec<String>) -> Self {
            Self {
                expected: logs,
                line: 0,
                buf: vec![],
            }
        }
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

    impl Write for LogAsserter {
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }

        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.extend_from_slice(buf);
            let log = String::from_utf8_lossy(&self.buf);
            if log.ends_with('\n') {
                if self.line >= self.expected.len() {
                    panic!("unexpected log line: {}", log)
                }
                // we skip 9 first characters because it is the hour
                // and the last one because it is \n
                let log = &log[9..log.len() - 1];
                assert_eq!(log, self.expected[self.line]);
                self.line += 1;
                self.buf = vec![];
            }
            Ok(buf.len())
        }
    }
}
