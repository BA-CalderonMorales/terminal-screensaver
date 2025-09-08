//! Terminal Screensaver
//! 
//! A dynamic terminal screen saver crate with plugin architecture that automatically
//! resizes with any screen size and provides extensible event handling.
//!
//! # Examples
//!
//! ```rust
//! use terminal_screensaver::{Config, run_screensaver};
//!
//! let config = Config {
//!     text: "Welcome to My App".to_string(),
//!     style: "default".to_string(),
//! };
//! 
//! // This will show a screen saver until ESC is pressed
//! run_screensaver(config);
//! ```

pub mod logger;
pub mod cli;
pub mod styles;
pub mod shared;
pub mod feature_alpha;

// Re-export commonly used items
pub use cli::cli_logic::Config;
pub use feature_alpha::feature_alpha_logic::run_screensaver;
