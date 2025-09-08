#[cfg(test)]
mod matrix_rain_tests {
    use super::matrix_rain_logic::MatrixRain;
    use ratatui::layout::Rect;

    #[test]
    fn test_matrix_rain_creation() {
        let matrix = MatrixRain::new(80, 24);
        assert_eq!(matrix.columns.len(), 40); // Every 2nd column
        assert!(matrix.update_interval.as_millis() > 0);
    }

    #[test]
    fn test_matrix_rain_adapts_to_size_changes() {
        let mut matrix = MatrixRain::new(80, 24);
        let initial_columns = matrix.columns.len();

        // Simulate size change
        matrix.update(120, 30);
        let new_columns = matrix.columns.len();

        assert_ne!(initial_columns, new_columns);
        assert_eq!(new_columns, 60); // 120 / 2 = 60 columns
    }

    #[test]
    fn test_matrix_rain_render_produces_output() {
        let matrix = MatrixRain::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let lines = matrix.render(area);

        assert_eq!(lines.len(), 10); // Should produce lines for each row
    }

    #[test]
    fn test_matrix_column_creation() {
        let column = MatrixColumn::new(5, 24);
        assert_eq!(column.x, 5);
        assert_eq!(column.y, 0);
        assert!(column.speed >= 1 && column.speed <= 3);
        assert!(column.length >= 5 && column.length <= 15);
        assert!(!column.characters.is_empty());
    }

    #[test]
    fn test_matrix_column_update() {
        let mut column = MatrixColumn::new(0, 10);
        let initial_y = column.y;
        column.update(10);

        // Y position should increase
        assert!(column.y >= initial_y);
    }

    #[test]
    fn test_matrix_column_wraps_around() {
        let mut column = MatrixColumn::new(0, 10);
        column.y = 25; // Set y beyond height + length
        column.update(10);

        // Should wrap around to 0
        assert_eq!(column.y, 0);
    }

    #[test]
    fn test_matrix_rain_handles_small_terminal() {
        let matrix = MatrixRain::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let lines = matrix.render(area);

        assert_eq!(lines.len(), 5);
        assert_eq!(matrix.columns.len(), 5); // 10 / 2 = 5 columns
    }

    #[test]
    fn test_matrix_rain_handles_large_terminal() {
        let matrix = MatrixRain::new(200, 80);
        let area = Rect::new(0, 0, 200, 80);
        let lines = matrix.render(area);

        assert_eq!(lines.len(), 80);
        assert_eq!(matrix.columns.len(), 100); // 200 / 2 = 100 columns
    }

    #[test]
    fn test_matrix_characters_are_valid() {
        let column = MatrixColumn::new(0, 24);
        let valid_chars = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";

        for &ch in &column.characters {
            assert!(valid_chars.contains(ch), "Invalid character found: {}", ch);
        }
    }
}
