use crate::cli::cli_logic::Config;
use crate::shared::shared_logic as shared;
use crate::shared::{SimpleRenderer, TextLine};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub fn run_screensaver(config: Config) {
    enable_raw_mode().unwrap();
    let mut renderer = SimpleRenderer::new().unwrap();
    shared::clear_screen();

    loop {
        renderer.update_size().unwrap();
        let (width, height) = renderer.get_size();

        // Create centered text
        let lines = create_text_display(&config.text, width, height);
        renderer.render_lines(lines).unwrap();

        if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            match code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    // Trigger additional options, but for now just log
                    log::info!("Enter pressed, triggering additional options");
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
}

fn create_text_display(text: &str, width: u16, height: u16) -> Vec<TextLine> {
    let mut lines = Vec::new();

    // Add empty lines to center vertically
    let center_y = height / 2;
    for y in 0..height {
        if y == center_y {
            // Center the text horizontally
            let padding = if text.len() < width as usize {
                (width as usize - text.len()) / 2
            } else {
                0
            };
            let centered_text = format!("{}{}", " ".repeat(padding), text);
            lines.push(TextLine::with_color(centered_text, Color::Green));
        } else {
            lines.push(TextLine::new(String::new()));
        }
    }

    lines
}
