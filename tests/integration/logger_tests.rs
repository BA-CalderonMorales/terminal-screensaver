use terminal_screensaver::logger::logger_macros::{get_log_level, set_log_level, LogLevel};

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

    set_log_level(LogLevel::Info);
    assert_eq!(get_log_level(), LogLevel::Info);
}

#[test]
fn test_log_level_comparison() {
    assert!(LogLevel::Error <= LogLevel::Error);
    assert!(LogLevel::Error <= LogLevel::Warn);
    assert!(LogLevel::Warn >= LogLevel::Error);
    assert!(LogLevel::Debug >= LogLevel::Info);
}

#[test]
fn test_logger_init() {
    // Test that init sets the default level
    terminal_screensaver::logger::logger_logic::init();
    assert_eq!(get_log_level(), LogLevel::Info);
}

#[test]
fn test_logger_init_with_level() {
    terminal_screensaver::logger::logger_logic::init_with_level(LogLevel::Warn);
    assert_eq!(get_log_level(), LogLevel::Warn);

    terminal_screensaver::logger::logger_logic::init_with_level(LogLevel::Debug);
    assert_eq!(get_log_level(), LogLevel::Debug);
}
