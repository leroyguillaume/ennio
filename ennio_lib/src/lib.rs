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
    use log::{set_logger, set_max_level, LevelFilter, Log, Metadata, Record};
    use std::{
        collections::HashMap,
        sync::{Mutex, Once},
        thread,
    };

    pub static mut LOGGER: Option<LoggerStub> = None;
    static mut LOGGER_LOCK: Option<Mutex<()>> = None;
    static mut LOG_INDEX: Option<HashMap<String, usize>> = None;
    static INIT: Once = Once::new();

    #[derive(Default)]
    pub struct LoggerStub {
        expected: HashMap<String, Vec<String>>,
    }

    impl LoggerStub {
        pub fn expect_logs(&mut self, logs: Vec<String>) {
            let thread = thread::current();
            let thread_name = thread.name().unwrap();
            unsafe {
                let _lock = LOGGER_LOCK.as_mut().unwrap().get_mut().unwrap();
                self.expected.insert(thread_name.into(), logs);
            }
        }
    }

    impl Log for LoggerStub {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn flush(&self) {}

        fn log(&self, record: &Record) {
            let thread = thread::current();
            let thread_name = thread.name().unwrap();
            unsafe {
                let _lock = LOGGER_LOCK.as_mut().unwrap().get_mut().unwrap();
                let logger = LOGGER.as_mut().unwrap();
                let expected = logger.expected.get(thread_name).unwrap();
                let log_index = LOG_INDEX.as_mut().unwrap();
                let index = *log_index.get(thread_name).unwrap_or(&0);
                if index >= expected.len() {
                    panic!("unexpected log: {}", record.args());
                }
                let expected = expected[index].clone();
                let log = record.args().to_string();
                assert_eq!(log, expected);
                log_index.insert(thread_name.into(), index + 1);
            }
        }
    }

    pub fn init_logger() {
        INIT.call_once(|| {
            let logger = LoggerStub::default();
            unsafe {
                LOGGER_LOCK = Some(Mutex::new(()));
                LOGGER = Some(logger);
                LOG_INDEX = Some(HashMap::new());
                set_logger(LOGGER.as_ref().unwrap()).unwrap();
                set_max_level(LevelFilter::Trace);
            };
        });
    }
}
