/*!
Copyright (c) 2022 Patineboot.
All rights reserved.

Elephant Archive is licensed under BSD 2-Clause License.
*/

use log::{Log, Record, Level, Metadata, SetLoggerError, LevelFilter};


pub struct ElephantLogger;

// impl ElephantLogger {
//     fn new() -> ElephantLogger {
//         let mut builder = Builder::from_env(FILTER_ENV);

//         ElephantLogger {
//             inner: builder.build(),
//         }
//     }

//     pub fn initialize() -> Result<(), SetLoggerError> {
//         let logger = Self::new();

//         log::set_max_level(logger.inner.filter());
//         log::set_boxed_logger(Box::new(logger))
//     }
// }

static DEFAULT_LOGGER: ElephantLogger = ElephantLogger;

impl ElephantLogger {
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
