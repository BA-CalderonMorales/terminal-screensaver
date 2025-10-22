// Features module - organized screensaver features following screaming architecture
//
// Each feature is self-contained in its own directory with:
// - feature_logic.rs: Core implementation
// - feature_logic_tests.rs: Comprehensive tests
// - mod.rs: Module declaration
//
// Active features:
// - text_display: Static and animated text display with script integration
// - bouncing_logo: Animated bouncing text/logo with physics

pub mod bouncing_logo;
pub mod text_display;

// Re-export common types for easier usage
pub use bouncing_logo::bouncing_logo_logic::BouncingLogoFeature;

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
    vec!["text_display", "bouncing_logo"]
}

// Feature descriptions for user interfaces and documentation
pub fn get_feature_description(feature_name: &str) -> Option<&'static str> {
    match feature_name {
        "text_display" => {
            Some("Static and animated text display with script integration and keyboard shortcuts")
        }
        "bouncing_logo" => Some("Animated bouncing text or logo with physics simulation"),
        _ => None,
    }
}
