use crate::cli::cli_logic::Config;
use crate::shared::shared_logic as shared;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

pub struct MatrixColumn {
    pub x: u16,
    pub y: u16,
    pub speed: u8,
    pub length: u8,
    pub characters: Vec<char>,
}

impl MatrixColumn {
    fn new(x: u16, _height: u16) -> Self {
        let mut rng = rand::rng();
        Self {
            x,
            y: 0,
            speed: rng.random_range(1..=3),
            length: rng.random_range(5..=15),
            characters: (0..15)
                .map(|_| {
                    let chars = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
                    let chars: Vec<char> = chars.chars().collect();
                    chars[rng.random_range(0..chars.len())]
                })
                .collect(),
        }
    }

    fn update(&mut self, height: u16) {
        self.y += self.speed as u16;
        if self.y > height + self.length as u16 {
            self.y = 0;
            let mut rng = rand::rng();
            self.speed = rng.random_range(1..=3);
            self.length = rng.random_range(5..=15);
            // Refresh some characters
            for i in 0..3 {
                if i < self.characters.len() {
                    let chars = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";
                    let chars: Vec<char> = chars.chars().collect();
                    self.characters[i] = chars[rng.random_range(0..chars.len())];
                }
            }
        }
    }
}

pub struct MatrixRain {
    pub columns: Vec<MatrixColumn>,
    pub last_update: Instant,
    pub update_interval: Duration,
}

impl MatrixRain {
    pub fn new(width: u16, height: u16) -> Self {
        let columns = (0..width)
            .step_by(2)
            .map(|x| MatrixColumn::new(x, height))
            .collect();

        Self {
            columns,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
        }
    }

    pub fn update(&mut self, width: u16, height: u16) {
        if self.last_update.elapsed() >= self.update_interval {
            // Adjust column count if terminal size changed
            let expected_columns = (width as usize).div_ceil(2);
            if self.columns.len() != expected_columns {
                self.columns = (0..width)
                    .step_by(2)
                    .map(|x| MatrixColumn::new(x, height))
                    .collect();
            }

            for column in &mut self.columns {
                column.update(height);
            }
            self.last_update = Instant::now();
        }
    }

    pub fn render(&self, area: Rect) -> Vec<Line> {
        let mut lines = vec![
            Line::from(vec![Span::raw(" ".repeat(area.width as usize))]);
            area.height as usize
        ];

        for column in &self.columns {
            if column.x >= area.width {
                continue;
            }

            for (i, &ch) in column.characters.iter().enumerate() {
                let char_y = column.y as i32 - i as i32;
                if char_y >= 0 && char_y < area.height as i32 {
                    let intensity = 1.0 - (i as f32 / column.length as f32);
                    let color = if i == 0 {
                        Color::White // Head of the trail
                    } else if intensity > 0.7 {
                        Color::Rgb(0, 255, 0) // Bright green
                    } else if intensity > 0.3 {
                        Color::Rgb(0, 180, 0) // Medium green
                    } else {
                        Color::Rgb(0, 100, 0) // Dark green
                    };

                    let line_index = char_y as usize;
                    if line_index < lines.len() && column.x < area.width {
                        let mut line_content = " ".repeat(area.width as usize);
                        line_content
                            .replace_range(column.x as usize..=column.x as usize, &ch.to_string());
                        lines[line_index] = Line::from(vec![Span::styled(
                            line_content,
                            Style::default().fg(color),
                        )]);
                    }
                }
            }
        }

        lines
    }
}

pub fn run_screensaver(config: Config) -> io::Result<()> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    shared::clear_screen();

    let size = terminal.size()?;
    let mut matrix = MatrixRain::new(size.width, size.height);

    loop {
        let size = terminal.size()?;
        matrix.update(size.width, size.height);

        terminal.draw(|f| {
            let area = f.area();
            let lines = matrix.render(area);

            let block = Block::default()
                .title(format!(" {} ", config.text))
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(Color::Green));

            let paragraph = Paragraph::new(lines)
                .block(block)
                .style(Style::default().bg(Color::Black));

            f.render_widget(paragraph, area);
        })?;

        // Non-blocking input check
        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        log::info!("Enter pressed in matrix_rain screensaver, triggering additional options");
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
