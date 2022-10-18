// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use log::{Log, Record, Level, Metadata, SetLoggerError, LevelFilter};

pub struct ElephantLogger;

static DEFAULT_LOGGER: ElephantLogger = ElephantLogger;

impl ElephantLogger {
    /// Initialize ElephantLogger 
    /// 
    /// # Examples
    /// ```
    /// use elephant_logger::ElephantLogger;
    /// elephant_logger::ElephantLogger::init().unwrap();
    /// ```
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&DEFAULT_LOGGER)
                .map(|()| log::set_max_level(LevelFilter::Info))
    }
}

impl Log for ElephantLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
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
