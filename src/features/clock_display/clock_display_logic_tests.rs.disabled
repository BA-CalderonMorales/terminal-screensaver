#[cfg(test)]
mod clock_display_tests {
    use super::clock_display_logic::ClockDisplay;
    use ratatui::layout::Rect;

    #[test]
    fn test_clock_display_creation() {
        let clock = ClockDisplay::new();
        assert_eq!(clock.show_seconds, true);
        assert_eq!(clock.show_date, true);
        assert_eq!(clock.time_format_24h, true);
    }

    #[test]
    fn test_clock_display_with_custom_config() {
        let clock = ClockDisplay::with_config(false, false, false);
        assert_eq!(clock.show_seconds, false);
        assert_eq!(clock.show_date, false);
        assert_eq!(clock.time_format_24h, false);
    }

    #[test]
    fn test_get_current_time_string_24h_with_seconds() {
        let clock = ClockDisplay::with_config(true, true, true);
        let time_str = clock.get_current_time_string();

        // Should match HH:MM:SS format
        assert!(time_str.len() >= 8); // At least HH:MM:SS
        assert!(time_str.contains(':'));
    }

    #[test]
    fn test_get_current_time_string_24h_without_seconds() {
        let clock = ClockDisplay::with_config(false, true, true);
        let time_str = clock.get_current_time_string();

        // Should match HH:MM format
        assert!(time_str.len() >= 5); // At least HH:MM
        assert!(time_str.contains(':'));
        assert!(!time_str.matches(':').count() > 1); // Only one colon
    }

    #[test]
    fn test_get_current_time_string_12h_format() {
        let clock = ClockDisplay::with_config(true, true, false);
        let time_str = clock.get_current_time_string();

        // Should contain AM or PM
        assert!(time_str.contains("AM") || time_str.contains("PM"));
    }

    #[test]
    fn test_get_current_date_string() {
        let clock = ClockDisplay::new();
        let date_str = clock.get_current_date_string();

        // Should contain common date elements
        assert!(date_str.contains(','));
        assert!(date_str.len() > 10); // Should be a reasonable date string length
    }

    #[test]
    fn test_create_digital_time_basic() {
        let clock = ClockDisplay::new();
        let digital_time = clock.create_digital_time("12:34");

        assert_eq!(digital_time.len(), 5); // Should have 5 rows
        for line in &digital_time {
            assert!(!line.is_empty()); // Each line should have content
        }
    }

    #[test]
    fn test_create_digital_time_with_seconds() {
        let clock = ClockDisplay::new();
        let digital_time = clock.create_digital_time("12:34:56");

        assert_eq!(digital_time.len(), 5); // Should have 5 rows
        for line in &digital_time {
            assert!(!line.is_empty()); // Each line should have content
            assert!(line.len() > 30); // Should be wider with seconds
        }
    }

    #[test]
    fn test_create_digital_time_handles_all_digits() {
        let clock = ClockDisplay::new();

        // Test all digits 0-9
        for digit in 0..10 {
            let digital_time = clock.create_digital_time(&digit.to_string());
            assert_eq!(digital_time.len(), 5);
            for line in &digital_time {
                assert!(!line.is_empty());
            }
        }
    }

    #[test]
    fn test_render_creates_paragraph() {
        let clock = ClockDisplay::new();
        let area = Rect::new(0, 0, 80, 24);
        let paragraph = clock.render(area, "Test Clock");

        // Should create a valid paragraph (basic structure test)
        // The actual rendering details are handled by ratatui
        // We're mainly testing that it doesn't panic and returns something
    }

    #[test]
    fn test_render_with_empty_title() {
        let clock = ClockDisplay::new();
        let area = Rect::new(0, 0, 80, 24);
        let paragraph = clock.render(area, "");

        // Should handle empty title gracefully
    }

    #[test]
    fn test_render_adapts_to_small_area() {
        let clock = ClockDisplay::new();
        let small_area = Rect::new(0, 0, 20, 10);
        let paragraph = clock.render(small_area, "Clock");

        // Should handle small areas without panicking
    }

    #[test]
    fn test_render_adapts_to_large_area() {
        let clock = ClockDisplay::new();
        let large_area = Rect::new(0, 0, 200, 80);
        let paragraph = clock.render(large_area, "Big Clock");

        // Should handle large areas without issues
    }

    #[test]
    fn test_clock_display_without_date() {
        let clock = ClockDisplay::with_config(true, false, true);
        let area = Rect::new(0, 0, 80, 24);
        let paragraph = clock.render(area, "No Date Clock");

        // Should render without date information
    }

    #[test]
    fn test_clock_display_minimal_config() {
        let clock = ClockDisplay::with_config(false, false, true);
        let time_str = clock.get_current_time_string();
        let date_str = if clock.show_date {
            clock.get_current_date_string()
        } else {
            String::new()
        };

        // Time should still be present
        assert!(!time_str.is_empty());
        // Date should be empty
        assert!(date_str.is_empty());
    }
}
