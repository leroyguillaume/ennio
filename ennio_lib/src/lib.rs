pub mod action;
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
    use crate::{action::*, context::*};
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

    pub struct LogAsserter {
        expected: Vec<String>,
        line: usize,
        buf: Vec<u8>,
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
