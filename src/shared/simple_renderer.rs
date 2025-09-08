use crossterm::{
    cursor,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};

pub struct SimpleRenderer {
    width: u16,
    height: u16,
}

#[derive(Clone)]
pub struct TextLine {
    pub content: String,
    pub color: Option<Color>,
}

impl TextLine {
    pub fn new(content: String) -> Self {
        Self {
            content,
            color: None,
        }
    }

    pub fn with_color(content: String, color: Color) -> Self {
        Self {
            content,
            color: Some(color),
        }
    }
}

impl SimpleRenderer {
    pub fn new() -> std::io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        Ok(Self { width, height })
    }

    pub fn clear_screen(&mut self) -> std::io::Result<()> {
        stdout()
            .execute(Clear(ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn render_lines(&mut self, lines: Vec<TextLine>) -> std::io::Result<()> {
        let mut stdout = stdout();

        // Clear screen and move to top
        stdout
            .queue(Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;

        // Render each line
        for (y, line) in lines.iter().enumerate() {
            if y >= self.height as usize {
                break;
            }

            stdout.queue(cursor::MoveTo(0, y as u16))?;

            if let Some(color) = line.color {
                stdout.queue(SetForegroundColor(color))?;
            }

            // Truncate line if it's too long
            let content = if line.content.len() > self.width as usize {
                &line.content[..self.width as usize]
            } else {
                &line.content
            };

            stdout.queue(Print(content))?;

            // Reset color if it was set
            if line.color.is_some() {
                stdout.queue(SetForegroundColor(Color::Reset))?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn update_size(&mut self) -> std::io::Result<()> {
        let (width, height) = crossterm::terminal::size()?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

#[derive(Clone)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_size(width: u16, height: u16) -> Self {
        Self::new(0, 0, width, height)
    }
}
