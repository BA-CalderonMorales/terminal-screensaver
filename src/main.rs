use terminal_screensaver::{logger, cli, feature_alpha};

fn main() {
    // Initialize logger
    terminal_screensaver::logger::logger_logic::init();

    // Parse CLI arguments
    let config = cli::cli_logic::parse_args();

    // Run the screen saver
    feature_alpha::feature_alpha_logic::run_screensaver(config);
}
