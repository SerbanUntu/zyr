use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, terminal};
use std::error::Error;
use std::io;

pub const FRAME_DURATION_MS: u64 = 16;
pub const PAGE_SIZE: u64 = 10;

pub struct RawTerminal {}

impl RawTerminal {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        terminal::enable_raw_mode().expect("Could not enable raw mode");
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            cursor::DisableBlinking,
            cursor::Hide,
        )?;
        Ok(Self {})
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            cursor::EnableBlinking,
            cursor::Show,
        )
        .expect("Could not restore the cursor settings");
    }
}
