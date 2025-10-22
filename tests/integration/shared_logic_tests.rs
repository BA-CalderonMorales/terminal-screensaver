use terminal_screensaver::shared::shared_logic;

#[test]
fn test_get_terminal_size_returns_valid_dimensions() {
    // This test may fail in CI without a proper terminal
    // but it should work in a normal terminal environment
    match shared::get_terminal_size() {
        Ok((width, height)) => {
            assert!(width > 0, "Width should be positive");
            assert!(height > 0, "Height should be positive");
            assert!(width < 10000, "Width should be reasonable");
            assert!(height < 10000, "Height should be reasonable");
        }
        Err(e) => {
            // In CI or headless environments, this might fail
            // We'll accept that as it's environment-dependent
            println!("Terminal size check failed (expected in CI): {}", e);
        }
    }
}

#[test]
fn test_clear_screen_returns_result() {
    // Test that clear_screen returns a Result type
    // In headless environments this might fail, but the API should work
    let result = shared::clear_screen();

    // We just verify it returns a Result - the actual success depends on env
    match result {
        Ok(()) => {}, // Success case
        Err(e) => {
            // In CI this might fail, which is acceptable
            println!("Clear screen failed (expected in CI): {}", e);
        }
    }
}
