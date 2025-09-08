use crossterm::execute;
use crossterm::terminal::{size, Clear, ClearType};
use std::io::stdout;

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn get_terminal_size() -> (u16, u16) {
    size().unwrap()
}
