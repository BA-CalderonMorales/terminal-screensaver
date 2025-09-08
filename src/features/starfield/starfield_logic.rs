use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};
use std::collections::HashMap;

#[derive(Clone)]
struct Star {
    x: f64,
    y: f64,
    z: f64,
    char: char,
    color: Color,
}

pub struct StarfieldFeature {
    stars: Vec<Star>,
    star_count: usize,
    speed: f32,
    #[allow(dead_code)]
    last_update: u64,
    area_width: u16,
    area_height: u16,
}

impl Default for StarfieldFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl StarfieldFeature {
    pub fn new() -> Self {
        Self {
            stars: Vec::new(),
            star_count: 100,
            speed: 1.0,
            last_update: 0,
            area_width: 80,
            area_height: 24,
        }
    }

    pub fn render(&mut self, area: Rect) -> Vec<Line> {
        // Initialize or resize stars if area changed
        if self.area_width != area.width || self.area_height != area.height || self.stars.is_empty()
        {
            self.area_width = area.width;
            self.area_height = area.height;
            self.initialize_stars();
        }

        self.update_stars();

        let mut lines = Vec::new();
        let mut star_map: HashMap<(usize, usize), Star> = HashMap::new();

        // Map stars to screen positions
        for star in &self.stars {
            let screen_x = ((star.x / star.z) * (area.width as f64 / 2.0)
                + (area.width as f64 / 2.0)) as usize;
            let screen_y = ((star.y / star.z) * (area.height as f64 / 2.0)
                + (area.height as f64 / 2.0)) as usize;

            if screen_x < area.width as usize && screen_y < area.height as usize {
                star_map.insert((screen_x, screen_y), star.clone());
            }
        }

        // Generate lines with stars
        for y in 0..area.height {
            let mut line_spans = Vec::new();
            let mut current_line = String::new();
            let mut current_color = Color::White;

            for x in 0..area.width {
                if let Some(star) = star_map.get(&(x as usize, y as usize)) {
                    // Finish current span if color changes
                    if !current_line.is_empty() && current_color != star.color {
                        line_spans.push(Span::styled(
                            current_line.clone(),
                            Style::default().fg(current_color),
                        ));
                        current_line.clear();
                    }

                    current_line.push(star.char);
                    current_color = star.color;
                } else {
                    // Finish current span if we need to add a space
                    if !current_line.is_empty() && current_color != Color::Black {
                        line_spans.push(Span::styled(
                            current_line.clone(),
                            Style::default().fg(current_color),
                        ));
                        current_line.clear();
                    }

                    current_line.push(' ');
                    current_color = Color::Black;
                }
            }

            // Add final span for the line
            if !current_line.is_empty() {
                line_spans.push(Span::styled(
                    current_line,
                    Style::default().fg(current_color),
                ));
            }

            lines.push(Line::from(line_spans));
        }

        // Add controls at bottom if there's space
        if area.height > 2 {
            let last_line_idx = lines.len() - 1;
            lines[last_line_idx] = Line::from(vec![Span::styled(
                center_text("Press ESC to exit | +/- for speed", area.width as usize),
                Style::default().fg(Color::Gray),
            )]);
        }

        lines
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> ScreensaverAction {
        match key_event.code {
            KeyCode::Esc => ScreensaverAction::Exit,
            KeyCode::Enter => ScreensaverAction::NextScreen,
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.speed = (self.speed + 0.2).min(5.0);
                ScreensaverAction::Continue
            }
            KeyCode::Char('-') => {
                self.speed = (self.speed - 0.2).max(0.1);
                ScreensaverAction::Continue
            }
            KeyCode::Char(' ') => {
                // Reset starfield
                self.initialize_stars();
                ScreensaverAction::Continue
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // Change star count
                self.star_count = if self.star_count > 50 { 25 } else { 150 };
                self.initialize_stars();
                ScreensaverAction::Continue
            }
            _ => ScreensaverAction::Continue,
        }
    }

    pub fn resize(&mut self, new_area: Rect) {
        self.area_width = new_area.width;
        self.area_height = new_area.height;
        self.initialize_stars();
    }

    fn initialize_stars(&mut self) {
        self.stars.clear();
        let star_chars = ['*', '·', '°', '+', '×'];
        let star_colors = [
            Color::White,
            Color::Yellow,
            Color::Cyan,
            Color::Blue,
            Color::Magenta,
        ];

        for _ in 0..self.star_count {
            let z = (rand_f64() * 20.0) + 1.0; // Distance from viewer
            let x = (rand_f64() - 0.5) * 100.0; // X position in 3D space
            let y = (rand_f64() - 0.5) * 100.0; // Y position in 3D space

            let char_idx = (rand_f64() * star_chars.len() as f64) as usize;
            let color_idx = (rand_f64() * star_colors.len() as f64) as usize;

            self.stars.push(Star {
                x,
                y,
                z,
                char: star_chars[char_idx.min(star_chars.len() - 1)],
                color: star_colors[color_idx.min(star_colors.len() - 1)],
            });
        }
    }

    fn update_stars(&mut self) {
        for star in &mut self.stars {
            star.z -= self.speed as f64;

            // Reset star when it gets too close
            if star.z <= 0.1 {
                star.z = 20.0;
                star.x = (rand_f64() - 0.5) * 100.0;
                star.y = (rand_f64() - 0.5) * 100.0;
            }
        }
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

// Simple pseudo-random number generator for consistent behavior
static mut RAND_SEED: u64 = 1;

fn rand_f64() -> f64 {
    unsafe {
        RAND_SEED = RAND_SEED.wrapping_mul(1103515245).wrapping_add(12345);
        (RAND_SEED as f64) / (u64::MAX as f64)
    }
}
