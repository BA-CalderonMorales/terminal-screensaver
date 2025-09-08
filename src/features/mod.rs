// Features module - organized screensaver features following screaming architecture
//
// Each feature is self-contained in its own directory with:
// - feature_logic.rs: Core implementation
// - feature_logic_tests.rs: Comprehensive tests
// - mod.rs: Module declaration
//
// Available features:
// - text_display: Static and animated text display
// - matrix_rain: Matrix-style character rain effect
// - clock_display: Digital clock with customizable formats
// - bouncing_logo: Animated bouncing text/logo
// - system_info: Live system information display
// - wave_animation: ASCII wave animations with physics
// - starfield: 3D starfield simulation

pub mod bouncing_logo;
// Temporarily disabled complex features that need ratatui conversion
// pub mod clock_display;
// pub mod matrix_rain;
// pub mod starfield;
// pub mod system_info;
pub mod text_display;
// pub mod wave_animation;

// Re-export common types for easier usage
// pub use text_display::text_display_logic::TextDisplay; // text_display uses functions, not structs
pub use bouncing_logo::bouncing_logo_logic::BouncingLogoFeature;
// Temporarily disabled exports for complex features
// pub use clock_display::clock_display_logic::ClockDisplay;
// pub use matrix_rain::matrix_rain_logic::MatrixRain;
// pub use starfield::starfield_logic::StarfieldFeature;
// pub use system_info::system_info_logic::SystemInfoFeature;
// pub use wave_animation::wave_animation_logic::WaveAnimationFeature;

// Common screensaver action enum (each feature should implement this)
#[derive(Debug, PartialEq, Clone)]
pub enum ScreensaverAction {
    Continue,
    Exit,
    NextScreen,
}

// Feature trait that all screensaver features should implement
pub trait ScreensaverFeature {
    fn render(&mut self, area: crate::shared::Rect) -> Vec<crate::shared::TextLine>;
    fn handle_input(&mut self, key_event: crossterm::event::KeyEvent) -> ScreensaverAction;
    fn resize(&mut self, new_area: crate::shared::Rect);
    fn name(&self) -> &'static str;
}

// Feature registry for dynamic feature loading
pub fn get_available_features() -> Vec<&'static str> {
    vec![
        "text_display",
        "bouncing_logo",
        // Temporarily disabled complex features
        // "matrix_rain",
        // "clock_display",
        // "system_info",
        // "wave_animation",
        // "starfield",
    ]
} // Feature descriptions for user interfaces and documentation
pub fn get_feature_description(feature_name: &str) -> Option<&'static str> {
    match feature_name {
        "text_display" => Some("Static and animated text display with customizable content"),
        "bouncing_logo" => Some("Animated bouncing text or logo with physics"),
        // Temporarily disabled features
        // "matrix_rain" => Some("Matrix-style falling character rain animation"),
        // "clock_display" => Some("Digital clock display with multiple format options"),
        // "system_info" => Some("Live system information and runtime statistics"),
        // "wave_animation" => Some("Smooth ASCII wave animations with physics simulation"),
        // "starfield" => Some("3D starfield simulation with depth and movement"),
        _ => None,
    }
}
