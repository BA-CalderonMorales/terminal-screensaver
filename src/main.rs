use terminal_screensaver::{cli, features};

fn main() {
    // Initialize logger
    terminal_screensaver::logger::logger_logic::init();

    // Parse CLI arguments
    let config = cli::cli_logic::parse_args();

    // Run the screen saver - using text_display feature by default
    features::text_display::text_display_logic::run_screensaver(config);
}
