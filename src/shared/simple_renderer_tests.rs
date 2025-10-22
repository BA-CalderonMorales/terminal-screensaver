#[cfg(test)]
mod tests {
    use super::super::{Rect, SimpleRenderer, TextLine};
    use crossterm::style::Color;

    #[test]
    fn test_textline_new() {
        let line = TextLine::new("Hello".to_string());
        assert_eq!(line.content, "Hello");
        assert!(line.color.is_none());
    }

    #[test]
    fn test_textline_with_color() {
        let line = TextLine::with_color("Colored".to_string(), Color::Red);
        assert_eq!(line.content, "Colored");
        assert_eq!(line.color, Some(Color::Red));
    }

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10, 20, 100, 50);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 50);
    }

    #[test]
    fn test_rect_from_size() {
        let rect = Rect::from_size(80, 24);
        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 80);
        assert_eq!(rect.height, 24);
    }

    #[test]
    fn test_simple_renderer_construction() {
        // This might fail in CI without a terminal, but tests the API
        match SimpleRenderer::new() {
            Ok(renderer) => {
                let (width, height) = renderer.get_size();
                assert!(width > 0);
                assert!(height > 0);
            }
            Err(e) => {
                // In CI or headless mode, this is expected
                println!("Renderer construction failed (expected in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_textline_clone() {
        let line = TextLine::with_color("Test".to_string(), Color::Blue);
        let cloned = line.clone();
        assert_eq!(line.content, cloned.content);
        assert_eq!(line.color, cloned.color);
    }

    #[test]
    fn test_rect_clone() {
        let rect = Rect::new(5, 10, 50, 25);
        let cloned = rect.clone();
        assert_eq!(rect.x, cloned.x);
        assert_eq!(rect.y, cloned.y);
        assert_eq!(rect.width, cloned.width);
        assert_eq!(rect.height, cloned.height);
    }
}
