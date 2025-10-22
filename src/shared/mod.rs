pub mod shared_logic;
pub mod simple_renderer;

#[cfg(test)]
mod simple_renderer_tests;

// Re-export commonly used items
pub use simple_renderer::{Rect, SimpleRenderer, TextLine};
