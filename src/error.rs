//! Error types for terminal-screensaver
//!
//! This module provides a comprehensive error handling system with proper
//! error propagation and informative error messages.

use std::fmt;
use std::io;

/// Main error type for terminal-screensaver operations
#[derive(Debug)]
pub enum ScreensaverError {
    /// IO error occurred
    Io(io::Error),

    /// Terminal operation failed
    Terminal(String),

    /// Configuration error
    Config(String),

    /// Rendering error
    Render(String),

    /// Feature error
    Feature(String),
}

impl fmt::Display for ScreensaverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScreensaverError::Io(e) => write!(f, "IO error: {}", e),
            ScreensaverError::Terminal(msg) => write!(f, "Terminal error: {}", msg),
            ScreensaverError::Config(msg) => write!(f, "Configuration error: {}", msg),
            ScreensaverError::Render(msg) => write!(f, "Render error: {}", msg),
            ScreensaverError::Feature(msg) => write!(f, "Feature error: {}", msg),
        }
    }
}

impl std::error::Error for ScreensaverError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ScreensaverError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ScreensaverError {
    fn from(err: io::Error) -> Self {
        ScreensaverError::Io(err)
    }
}

/// Result type alias for screensaver operations
pub type Result<T> = std::result::Result<T, ScreensaverError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ScreensaverError::Config("invalid config".to_string());
        assert_eq!(err.to_string(), "Configuration error: invalid config");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: ScreensaverError = io_err.into();
        assert!(matches!(err, ScreensaverError::Io(_)));
    }

    #[test]
    fn test_error_source() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let err = ScreensaverError::Io(io_err);
        assert!(err.source().is_some());
    }
}
