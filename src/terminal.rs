use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, terminal};
use std::error::Error;
use std::io;

/// How often the raw terminal polls for user input, in milliseconds.
pub const FRAME_DURATION_MS: u64 = 16;

/// How many items (rows) to display per page in the TUI.
pub const PAGE_SIZE: u64 = 10;

/// Switch the terminal to raw mode when instantiated and reset it when dropped.
///
/// Remove the need to write setup and teardown code when working with the terminal in raw mode.
pub struct RawTerminal {}

impl RawTerminal {
    /// Switch the terminal to raw mode for the duration of the current scope (or until the
    /// object is explicitly dropped).
    ///
    /// # Examples
    ///
    /// ```
    /// let _raw_terminal = RawTerminal::new();
    /// // ...
    /// drop(_raw_terminal); // Back to canonical mode
    /// ```
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
