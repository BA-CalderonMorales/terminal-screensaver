// Zero-dependency logging macros for terminal-screensaver
// Replaces log + simplelog dependencies with simple, focused functionality

use std::io::{self, Write};

/// Log level for controlling output verbosity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
}

/// Global log level (can be set at runtime)
static mut GLOBAL_LOG_LEVEL: LogLevel = LogLevel::Info;

/// Set the global log level
pub fn set_log_level(level: LogLevel) {
    unsafe {
        GLOBAL_LOG_LEVEL = level;
    }
}

/// Get the current log level
pub fn get_log_level() -> LogLevel {
    unsafe { GLOBAL_LOG_LEVEL }
}

/// Internal logging function
#[doc(hidden)]
pub fn _log(level: LogLevel, args: std::fmt::Arguments) {
    if level <= get_log_level() {
        let prefix = match level {
            LogLevel::Error => "[ERROR]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Info => "[INFO]",
            LogLevel::Debug => "[DEBUG]",
        };

        let _ = writeln!(io::stderr(), "{} {}", prefix, args);
    }
}

/// Log an error message
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::logger_macros::_log(
            $crate::logger::logger_macros::LogLevel::Error,
            format_args!($($arg)*)
        )
    };
}

/// Log a warning message
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::logger_macros::_log(
            $crate::logger::logger_macros::LogLevel::Warn,
            format_args!($($arg)*)
        )
    };
}

/// Log an info message
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::logger_macros::_log(
            $crate::logger::logger_macros::LogLevel::Info,
            format_args!($($arg)*)
        )
    };
}

/// Log a debug message
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::logger_macros::_log(
            $crate::logger::logger_macros::LogLevel::Debug,
            format_args!($($arg)*)
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
    }

    #[test]
    fn test_set_and_get_log_level() {
        set_log_level(LogLevel::Debug);
        assert_eq!(get_log_level(), LogLevel::Debug);

        set_log_level(LogLevel::Error);
        assert_eq!(get_log_level(), LogLevel::Error);
    }
}
