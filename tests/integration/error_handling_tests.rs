use terminal_screensaver::error::{Result, ScreensaverError};

#[test]
fn test_error_display_formats() {
    let config_err = ScreensaverError::Config("invalid toml".to_string());
    assert_eq!(config_err.to_string(), "Configuration error: invalid toml");

    let terminal_err = ScreensaverError::Terminal("size failed".to_string());
    assert_eq!(terminal_err.to_string(), "Terminal error: size failed");

    let render_err = ScreensaverError::Render("rendering failed".to_string());
    assert_eq!(render_err.to_string(), "Render error: rendering failed");

    let feature_err = ScreensaverError::Feature("plugin error".to_string());
    assert_eq!(feature_err.to_string(), "Feature error: plugin error");
}

#[test]
fn test_io_error_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let screensaver_err: ScreensaverError = io_err.into();

    match screensaver_err {
        ScreensaverError::Io(_) => {}, // Expected
        _ => panic!("Expected Io error variant"),
    }
}

#[test]
fn test_result_type_alias() {
    fn returns_ok() -> Result<i32> {
        Ok(42)
    }

    fn returns_err() -> Result<i32> {
        Err(ScreensaverError::Config("test error".to_string()))
    }

    assert_eq!(returns_ok().unwrap(), 42);
    assert!(returns_err().is_err());
}

#[test]
fn test_error_source_chain() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let wrapped_err = ScreensaverError::Io(io_err);

    assert!(wrapped_err.source().is_some());
}
