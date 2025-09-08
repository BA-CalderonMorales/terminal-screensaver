use crate::cli::cli_logic::Config;
use crate::shared::shared_logic as shared;
use crate::shared::{SimpleRenderer, TextLine, Rect};
use chrono::{DateTime, Local};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::style::Color;
use std::io;
use std::time::{Duration, SystemTime};

pub struct ClockDisplay {
    pub show_seconds: bool,
    pub show_date: bool,
    pub time_format_24h: bool,
    pub color: Color,
    pub last_update: SystemTime,
}

impl Default for ClockDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl ClockDisplay {
    pub fn new() -> Self {
        Self {
            show_seconds: true,
            show_date: true,
            time_format_24h: true,
            color: Color::Green,
            last_update: SystemTime::now(),
        }
    }

    pub fn with_config(show_seconds: bool, show_date: bool, time_format_24h: bool) -> Self {
        Self {
            show_seconds,
            show_date,
            time_format_24h,
            color: Color::Green,
            last_update: SystemTime::now(),
        }
    }

    fn get_current_time_string(&self) -> String {
        let now: DateTime<Local> = Local::now();

        let time_str = if self.time_format_24h {
            if self.show_seconds {
                now.format("%H:%M:%S").to_string()
            } else {
                now.format("%H:%M").to_string()
            }
        } else if self.show_seconds {
            now.format("%I:%M:%S %p").to_string()
        } else {
            now.format("%I:%M %p").to_string()
        };

        time_str
    }

    fn get_current_date_string(&self) -> String {
        let now: DateTime<Local> = Local::now();
        now.format("%A, %B %d, %Y").to_string()
    }

    fn create_digital_time(&self, time_str: &str) -> Vec<String> {
        let digits = [
            [" ███ ", "█   █", "█   █", "█   █", " ███ "], // 0
            ["  █  ", " ██  ", "  █  ", "  █  ", " ███ "], // 1
            [" ███ ", "    █", " ███ ", "█    ", " ███ "], // 2
            [" ███ ", "    █", " ███ ", "    █", " ███ "], // 3
            ["█   █", "█   █", " ███ ", "    █", "    █"], // 4
            [" ███ ", "█    ", " ███ ", "    █", " ███ "], // 5
            [" ███ ", "█    ", " ███ ", "█   █", " ███ "], // 6
            [" ███ ", "    █", "   █ ", "  █  ", " █   "], // 7
            [" ███ ", "█   █", " ███ ", "█   █", " ███ "], // 8
            [" ███ ", "█   █", " ███ ", "    █", " ███ "], // 9
            ["     ", "  █  ", "     ", "  █  ", "     "], // : (colon)
            ["     ", "     ", "     ", "     ", "     "], // (space)
        ];

        let mut result = vec![String::new(); 5];

        for ch in time_str.chars() {
            let digit_index = match ch {
                '0'..='9' => (ch as usize) - ('0' as usize),
                ':' => 10,
                _ => 11, // Space or other characters
            };

            for (i, line) in digits[digit_index].iter().enumerate() {
                result[i].push_str(line);
                result[i].push(' '); // Add spacing between digits
            }
        }

        result
    }

    pub fn render<'a>(&self, area: Rect, title: &'a str) -> Paragraph<'a> {
        let time_str = self.get_current_time_string();
        let date_str = if self.show_date {
            self.get_current_date_string()
        } else {
            String::new()
        };

        let digital_time = self.create_digital_time(&time_str);

        let mut lines = Vec::new();

        // Add some vertical centering space
        let vertical_padding = (area.height as usize).saturating_sub(10) / 2;
        for _ in 0..vertical_padding {
            lines.push(Line::from(""));
        }

        // Add title if provided
        if !title.is_empty() {
            lines.push(Line::from(Span::styled(
                title,
                Style::default().fg(Color::Cyan),
            )));
            lines.push(Line::from(""));
        }

        // Add digital time display
        for line in digital_time {
            lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::White),
            )));
        }

        // Add date if enabled
        if self.show_date && !date_str.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                date_str,
                Style::default().fg(Color::Yellow),
            )));
        }

        // Add timezone info
        let now: DateTime<Local> = Local::now();
        let timezone_str = now.format("%Z").to_string();
        if !timezone_str.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                timezone_str,
                Style::default().fg(Color::Gray),
            )));
        }

        Paragraph::new(lines).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Blue)),
        )
    }
}

pub fn run_screensaver(config: Config) -> io::Result<()> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    shared::clear_screen();

    let clock = ClockDisplay::new();

    loop {
        terminal.draw(|f| {
            let area = f.area();
            let clock_widget = clock.render(area, &config.text);
            f.render_widget(clock_widget, area);
        })?;

        // Non-blocking input check with shorter timeout for clock updates
        if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        log::info!("Enter pressed in clock_display screensaver, triggering additional options");
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
