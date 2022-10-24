// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use log::{Log, Record, Level, Metadata, SetLoggerError, LevelFilter};

pub struct Logger;

static DEFAULT_LOGGER: Logger = Logger;

// #[macro_export(local_inner_macros)]
#[macro_export]
macro_rules! print_macro {
    (target: $target:expr, $($arg:tt)+) => (log::log!(target: $target, log::Level::Info, $($arg)+));
    ($($arg:tt)+) => (log::log!(log::Level::Info, $($arg)+))
}

impl Logger {
    /// Initialize ElephantLogger 
    /// 
    /// # Examples
    /// ```
    /// use elephant_logger::ElephantLogger;
    /// elephant_logger::ElephantLogger::init().unwrap();
    /// ```
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&DEFAULT_LOGGER)
                .map(|()| log::set_max_level(LevelFilter::Debug))
    }

    pub fn displayaaa() {
        print_macro!("bbbbbbb");
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = match record.level() {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
        };

        println!("{} - {}", level, record.args());
    }

    fn flush(&self) {}
}
