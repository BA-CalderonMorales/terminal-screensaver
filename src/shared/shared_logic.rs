use crossterm::execute;
use crossterm::terminal::{size, Clear, ClearType};
use std::io::stdout;
use crate::error::{Result, ScreensaverError};

pub fn clear_screen() -> Result<()> {
    execute!(stdout(), Clear(ClearType::All))
        .map_err(|e| ScreensaverError::Terminal(format!("Failed to clear screen: {}", e)))?;
    Ok(())
}

pub fn get_terminal_size() -> Result<(u16, u16)> {
    size().map_err(|e| ScreensaverError::Terminal(format!("Failed to get terminal size: {}", e)))
}
