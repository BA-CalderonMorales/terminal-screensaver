use super::logger_macros::{set_log_level, LogLevel};

/// Initialize the logger with default settings
/// This is now a lightweight function that just sets the log level
pub fn init() {
    // Set default log level to Info
    set_log_level(LogLevel::Info);
}

/// Initialize the logger with a specific log level
pub fn init_with_level(level: LogLevel) {
    set_log_level(level);
}
