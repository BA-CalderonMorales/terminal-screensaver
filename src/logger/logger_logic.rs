use simplelog::*;
use std::fs::File;

pub fn init() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("screensaver.log").unwrap(),
        ),
    ])
    .unwrap();
}
