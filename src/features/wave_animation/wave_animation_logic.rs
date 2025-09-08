use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};
use std::f64::consts::PI;

pub struct WaveAnimationFeature {
    time: f64,
    speed: f64,
    amplitude: f64,
    frequency: f64,
    #[allow(dead_code)]
    phase_shift: f64,
    wave_chars: Vec<char>,
}

impl Default for WaveAnimationFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveAnimationFeature {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            speed: 0.1,
            amplitude: 8.0,
            frequency: 0.3,
            phase_shift: 0.0,
            wave_chars: vec!['~', '`', '^', '*', '°', '·', ' '],
        }
    }

    pub fn render(&mut self, area: Rect) -> Vec<Line> {
        self.time += self.speed;
        let mut lines = Vec::new();

        let center_y = area.height as f64 / 2.0;
        let width = area.width as usize;

        // Title
        if area.height > 5 {
            lines.push(Line::from(vec![Span::styled(
                center_text("ASCII Wave Animation", width),
                Style::default().fg(Color::Cyan),
            )]));
            lines.push(Line::from(""));
        }

        // Generate wave lines
        for y in 0..area.height {
            if lines.len() >= area.height as usize {
                break;
            }

            // Skip title area
            if area.height > 5 && y < 2 {
                continue;
            }

            let mut wave_line = String::new();

            for x in 0..width {
                let x_normalized = x as f64 / width as f64;
                let wave_value = (self.frequency * x_normalized * 2.0 * PI + self.time).sin();
                let wave_y = center_y + self.amplitude * wave_value;

                let distance_to_wave = (y as f64 - wave_y).abs();

                // Create wave effect with different characters based on distance
                let char_index = if distance_to_wave < 0.5 {
                    0 // Main wave character
                } else if distance_to_wave < 1.5 {
                    1 // Near wave
                } else if distance_to_wave < 2.5 {
                    2 // Medium distance
                } else if distance_to_wave < 3.5 {
                    3 // Far distance
                } else if distance_to_wave < 4.5 {
                    4 // Very far
                } else if distance_to_wave < 5.5 {
                    5 // Fading
                } else {
                    6 // Background (space)
                };

                wave_line.push(self.wave_chars[char_index.min(self.wave_chars.len() - 1)]);
            }

            // Color the wave based on position
            let color = if y as f64 > center_y {
                Color::Blue
            } else {
                Color::Cyan
            };

            lines.push(Line::from(vec![Span::styled(
                wave_line,
                Style::default().fg(color),
            )]));
        }

        // Add controls at bottom if there's space
        if lines.len() < area.height as usize - 1 {
            lines.push(Line::from(vec![Span::styled(
                center_text("Press ESC to exit | SPACE to change wave", width),
                Style::default().fg(Color::Gray),
            )]));
        }

        lines
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> ScreensaverAction {
        match key_event.code {
            KeyCode::Esc => ScreensaverAction::Exit,
            KeyCode::Enter => ScreensaverAction::NextScreen,
            KeyCode::Char(' ') => {
                // Change wave parameters for variety
                self.speed = if self.speed > 0.05 { 0.02 } else { 0.15 };
                self.frequency = if self.frequency > 0.2 { 0.1 } else { 0.5 };
                self.amplitude = if self.amplitude > 5.0 { 3.0 } else { 10.0 };
                ScreensaverAction::Continue
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                // Increase speed
                self.speed = (self.speed + 0.01).min(0.5);
                ScreensaverAction::Continue
            }
            KeyCode::Char('-') => {
                // Decrease speed
                self.speed = (self.speed - 0.01).max(0.01);
                ScreensaverAction::Continue
            }
            _ => ScreensaverAction::Continue,
        }
    }

    pub fn resize(&mut self, _new_area: Rect) {
        // Wave automatically adapts to new dimensions during render
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreensaverAction {
    Continue,
    Exit,
    NextScreen,
}

fn center_text(text: &str, width: usize) -> String {
    let text_len = text.len();
    if text_len >= width {
        text.to_string()
    } else {
        let padding = (width - text_len) / 2;
        format!("{}{}", " ".repeat(padding), text)
    }
}
