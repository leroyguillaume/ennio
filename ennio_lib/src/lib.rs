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
    use std::io::{self, Write};

    pub struct LogAsserter {
        expected: Vec<String>,
        line: usize,
        buf: Vec<u8>,
    }

    impl LogAsserter {
        pub fn new(logs: Vec<String>) -> Self {
            Self {
                expected: logs,
                line: 0,
                buf: vec![],
            }
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
