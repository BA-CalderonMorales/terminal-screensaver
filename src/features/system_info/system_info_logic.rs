use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SystemInfoFeature {
    content: Vec<String>,
    last_update: u64,
    update_interval: u64, // seconds
}

impl Default for SystemInfoFeature {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfoFeature {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            last_update: 0,
            update_interval: 5, // Update every 5 seconds
        }
    }

    pub fn render(&mut self, area: Rect) -> Vec<Line> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update system info if interval passed
        if now - self.last_update >= self.update_interval {
            self.update_system_info();
            self.last_update = now;
        }

        let mut lines = Vec::new();

        // System Information Header
        lines.push(Line::from(vec![Span::styled(
            "=== SYSTEM INFORMATION ===",
            Style::default().fg(Color::Cyan),
        )]));
        lines.push(Line::from(""));

        // Display collected system info
        for info_line in &self.content {
            lines.push(Line::from(vec![Span::styled(
                info_line,
                Style::default().fg(Color::White),
            )]));
        }

        // Pad remaining space
        while lines.len() < area.height as usize {
            lines.push(Line::from(""));
        }

        lines
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> ScreensaverAction {
        match key_event.code {
            KeyCode::Esc => ScreensaverAction::Exit,
            KeyCode::Enter => ScreensaverAction::NextScreen,
            KeyCode::Char('r') => {
                // Force refresh
                self.last_update = 0;
                ScreensaverAction::Continue
            }
            _ => ScreensaverAction::Continue,
        }
    }

    pub fn resize(&mut self, _new_area: Rect) {
        // System info doesn't need special resize handling
    }

    fn update_system_info(&mut self) {
        self.content.clear();

        // Current time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hours = (now / 3600) % 24;
        let minutes = (now / 60) % 60;
        let seconds = now % 60;

        self.content.push(format!(
            "Current Time: {}:{}:{}",
            format_two_digits(hours),
            format_two_digits(minutes),
            format_two_digits(seconds)
        ));

        // Uptime (simplified calculation)
        self.content.push(format!("Uptime: {} seconds", now));

        // Memory info (placeholder)
        self.content
            .push("Memory: Information unavailable".to_string());

        // CPU info (placeholder)
        self.content
            .push("CPU: Information unavailable".to_string());

        // System name
        self.content
            .push(format!("System: {}", std::env::consts::OS));
        self.content
            .push(format!("Architecture: {}", std::env::consts::ARCH));

        // Environment variables count
        let env_count = std::env::vars().count();
        self.content
            .push(format!("Environment Variables: {}", env_count));

        // Current directory
        if let Ok(current_dir) = std::env::current_dir() {
            self.content
                .push(format!("Current Dir: {}", current_dir.display()));
        }

        // Add some spacing
        self.content.push("".to_string());
        self.content
            .push("Press 'r' to refresh, ESC to exit".to_string());
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreensaverAction {
    Continue,
    Exit,
    NextScreen,
}

fn format_two_digits(num: u64) -> String {
    if num < 10 {
        format!("0{}", num)
    } else {
        format!("{}", num)
    }
}
