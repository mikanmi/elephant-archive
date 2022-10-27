// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use once_cell::sync::OnceCell;
use chrono::Local;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    None = 1,
    Error,
    Warn,
    Display,
    Info,
    Debug,
    Trace,
}

pub struct Attribute {
    level: Level,
    file: String,
    line: u32,
}

#[derive(Debug)]
pub struct Logger {
    log_level: Level,
}

#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)*) => {{
        let attribute = $crate::Attribute::new(
            $lvl,
            std::file!(),
            std::line!(),
        );
        let message = std::format!($($arg)*);
        $crate::Logger::log(message, attribute);
    }};
}

#[macro_export]
macro_rules! display {
    () => {{
        $crate::log!($crate::Level::Display, "");
    }};
    ($($arg:tt)*) => {{
        $crate::log!($crate::Level::Display, $($arg)*)
    }}
}


#[macro_export]
macro_rules! error {
    () => {{
        $crate::log!($crate::Level::Error, "");
    }};
    ($($arg:tt)*) => {{
        $crate::log!($crate::Level::Error, $($arg)*)
    }}
}

#[macro_export]
macro_rules! warn {
    () => {{
        $crate::log!($crate::Level::Warn, "");
    }};
    ($($arg:tt)*) => {{
        $crate::log!($crate::Level::Warn, $($arg)*)
    }}
}

#[macro_export]
macro_rules! info {
    () => {{
        $crate::log!($crate::Level::Info, "");
    }};
    ($($arg:tt)*) => {{
        $crate::log!($crate::Level::Info, $($arg)*)
    }}
}

#[macro_export]
macro_rules! debug {
    () => {{
        $crate::log!($crate::Level::Debug, "");
    }};
    ($($arg:tt)*) => {{
        $crate::log!($crate::Level::Debug, $($arg)*)
    }}
}

#[macro_export]
macro_rules! trace {
    () => {{
        $crate::log!($crate::Level::Trace, "");
    }};
    ($($arg:tt)*) => {{
        $crate::log!($crate::Level::Trace, $($arg)*)
    }}
}

impl Attribute {
    pub fn new(level: Level, file: &str, line: u32) -> Attribute {
        Attribute {
            level,
            file: file.to_string(),
            line,
        }
    }
}

static DEFAULT_LOGGER: OnceCell<Logger> = OnceCell::new();

impl Logger {

    pub fn init(level: Level) {
        let logger = Logger {
            log_level: level,
        };
        DEFAULT_LOGGER.set(logger).unwrap();
    }

    pub fn log(message: String, attribute: Attribute) {
        let logger = DEFAULT_LOGGER.get().unwrap();

        if attribute.level > logger.log_level {
            return;
        }

        let level = match attribute.level {
            Level::Error => "ERR",
            Level::Warn => "WRN",
            Level::Display => "DSP",
            Level::Info => "INF",
            Level::Debug => "DBG",
            Level::Trace => "TRC",
            Level::None => "NON",
        };

        let datetime = Local::now().format("%FT%T%.3f");

        println!("[{}][{}]{}:{} {}", datetime, level, attribute.file, attribute.line, message);
    }

    pub fn set_level(&mut self, level: Level) {
        self.log_level = level;
    }
}
