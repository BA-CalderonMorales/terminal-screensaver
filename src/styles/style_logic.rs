use crossterm::style::{Attribute, Color};

pub struct Style {
    pub color: Option<Color>,
    pub bg_color: Option<Color>,
    pub attributes: Vec<Attribute>,
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Style {
    pub fn new() -> Self {
        Self {
            color: None,
            bg_color: None,
            attributes: Vec::new(),
        }
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }
}

pub fn get_style(style_name: &str) -> Style {
    match style_name {
        "default" => Style::new().fg(Color::White),
        "red" => Style::new().fg(Color::Red),
        "blue" => Style::new().fg(Color::Blue),
        "green" => Style::new().fg(Color::Green),
        _ => Style::new().fg(Color::White),
    }
}
