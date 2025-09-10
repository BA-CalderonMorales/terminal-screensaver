//! Terminal Screensaver
//!
//! A dynamic terminal screen saver crate with plugin architecture that automatically
//! resizes with any screen size and provides extensible event handling.
//!
//! # Examples
//!
//! ```rust
//! use terminal_screensaver::Config;
//!
//! let config = Config {
//!     text: "Welcome to My App".to_string(),
//!     style: "default".to_string(),
//!     actions: vec![],
//! };
//!
//! // Config is available for use with screensaver features
//! assert_eq!(config.text, "Welcome to My App");
//! ```

pub mod cli;
pub mod features;
pub mod logger;
pub mod services;
pub mod shared;
pub mod styles;

// Re-export commonly used items
pub use cli::cli_logic::Config;
pub use features::{
    get_available_features,
    get_feature_description,
    // Temporarily disabled complex features
    // TextDisplayFeature, MatrixRainFeature, ClockDisplayFeature,
    // SystemInfoFeature, WaveAnimationFeature, StarfieldFeature,
    BouncingLogoFeature,
    ScreensaverAction,
    ScreensaverFeature,
};
