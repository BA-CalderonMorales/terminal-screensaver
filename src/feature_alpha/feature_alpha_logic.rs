use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use crate::cli::cli_logic::Config;
use crate::shared::shared_logic as shared;
use crate::styles::style_logic as styles;

pub fn run_screensaver(config: Config) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    shared::clear_screen();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let text = config.text.as_str();
            let style = styles::get_style(&config.style);
            let paragraph = ratatui::widgets::Paragraph::new(text)
                .style(style)
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(paragraph, size);
        }).unwrap();

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
