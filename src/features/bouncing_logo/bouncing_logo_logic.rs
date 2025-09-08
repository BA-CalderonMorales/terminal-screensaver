use crate::shared::{Rect, TextLine};
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::Color;

pub struct BouncingLogoFeature {
    text: String,
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    color: Color,
}

impl Default for BouncingLogoFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl BouncingLogoFeature {
    pub fn new() -> Self {
        Self {
            text: "Terminal Screensaver".to_string(),
            x: 10.0,
            y: 5.0,
            velocity_x: 1.0,
            velocity_y: 0.5,
            color: Color::Green,
        }
    }

    pub fn render(&mut self, area: Rect) -> Vec<TextLine> {
        // Update position
        self.update_position(area.clone());

        let mut lines = Vec::new();

        // Create empty lines up to the logo position
        for y in 0..area.height {
            if y == self.y as u16
                && self.x >= 0.0
                && (self.x as usize + self.text.len()) <= area.width as usize
            {
                // Create line with logo at the correct position
                let padding = " ".repeat(self.x as usize);
                let line_content = format!("{}{}", padding, self.text);
                lines.push(TextLine::with_color(line_content, self.color));
            } else {
                lines.push(TextLine::new(String::new()));
            }
        }

        // Add controls at bottom if there's space
        if area.height > 2 {
            let controls = "Press ESC to exit | SPACE to change color";
            let padding = if controls.len() < area.width as usize {
                (area.width as usize - controls.len()) / 2
            } else {
                0
            };

            if let Some(last_line) = lines.last_mut() {
                *last_line = TextLine::with_color(
                    format!("{}{}", " ".repeat(padding), controls),
                    Color::DarkGrey,
                );
            }
        }

        lines
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> ScreensaverAction {
        match key_event.code {
            KeyCode::Esc => ScreensaverAction::Exit,
            KeyCode::Enter => ScreensaverAction::NextScreen,
            KeyCode::Char(' ') => {
                // Change color
                self.color = match self.color {
                    Color::Green => Color::Cyan,
                    Color::Cyan => Color::Yellow,
                    Color::Yellow => Color::Magenta,
                    Color::Magenta => Color::Red,
                    Color::Red => Color::Blue,
                    Color::Blue => Color::White,
                    _ => Color::Green,
                };
                ScreensaverAction::Continue
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                // Increase speed
                self.velocity_x *= 1.2;
                self.velocity_y *= 1.2;
                ScreensaverAction::Continue
            }
            KeyCode::Char('-') => {
                // Decrease speed
                self.velocity_x /= 1.2;
                self.velocity_y /= 1.2;
                ScreensaverAction::Continue
            }
            _ => ScreensaverAction::Continue,
        }
    }

    pub fn resize(&mut self, new_area: Rect) {
        // Adjust position if outside new boundaries
        if self.x + self.text.len() as f64 >= new_area.width as f64 {
            self.x = (new_area.width as f64 - self.text.len() as f64).max(0.0);
        }
        if self.y >= new_area.height as f64 {
            self.y = (new_area.height as f64 - 1.0).max(0.0);
        }
    }

    fn update_position(&mut self, area: Rect) {
        // Update position
        self.x += self.velocity_x;
        self.y += self.velocity_y;

        // Bounce off edges
        let text_width = self.text.len() as f64;

        if self.x <= 0.0 || self.x + text_width >= area.width as f64 {
            self.velocity_x = -self.velocity_x;
            self.x = self.x.max(0.0).min(area.width as f64 - text_width);
        }

        if self.y <= 0.0 || self.y >= area.height as f64 - 1.0 {
            self.velocity_y = -self.velocity_y;
            self.y = self.y.max(0.0).min(area.height as f64 - 1.0);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreensaverAction {
    Continue,
    Exit,
    NextScreen,
}
