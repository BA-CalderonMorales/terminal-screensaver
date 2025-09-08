use ratatui::style::{Color, Style};

pub fn get_style(style_name: &str) -> Style {
    match style_name {
        "default" => Style::default().fg(Color::White),
        "red" => Style::default().fg(Color::Red),
        "blue" => Style::default().fg(Color::Blue),
        "green" => Style::default().fg(Color::Green),
        _ => Style::default().fg(Color::White),
    }
}
