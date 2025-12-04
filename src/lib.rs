use crossterm::{cursor, queue, terminal};
use std::io::{self, Write};

pub struct WindowSetup {
    capture_keyboard: bool,
    alternate_screen: bool,
    show_cursor: bool,
}

impl WindowSetup {
    pub fn default() -> Self {
        WindowSetup {
            capture_keyboard: true,
            alternate_screen: true,
            show_cursor: false,
        }
    }

    pub fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        return self;
    }

    pub fn capture_keyboard(mut self, capture: bool) -> Self {
        self.capture_keyboard = capture;
        return self;
    }

    pub fn alternate_screen(mut self, use_alternate: bool) -> Self {
        self.alternate_screen = use_alternate;
        return self;
    }
}

pub struct TerminalContext {
    raw_mode: bool,
    alternate_screen: bool,
    show_cursor: bool,
}

impl TerminalContext {
    /// Creates a new context for the terminal
    /// with the settings specified in setup
    pub fn new(setup: WindowSetup) -> Result<Self, io::Error> {
        let mut stdout = io::stdout();
        if setup.capture_keyboard {
            terminal::enable_raw_mode()?;
        }
        if setup.alternate_screen {
            queue!(stdout, terminal::EnterAlternateScreen)?;
        }
        if setup.show_cursor == false {
            queue!(stdout, cursor::Hide)?;
        }
        stdout.flush()?;
        return Ok(Self {
            raw_mode: setup.capture_keyboard,
            alternate_screen: setup.alternate_screen,
            show_cursor: setup.show_cursor,
        });
    }
}

impl Drop for TerminalContext {
    fn drop(&mut self) {
        let mut stdout = io::stdout();
        if self.raw_mode {
            let _ = terminal::disable_raw_mode();
        }
        if self.alternate_screen {
            let _ = queue!(stdout, terminal::LeaveAlternateScreen);
        }
        if self.show_cursor == false {
            let _ = queue!(stdout, cursor::Show);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_mode_supported() {
        let _ = TerminalContext::new(
            WindowSetup::default()
                .show_cursor(true)
                .alternate_screen(false),
        )
        .unwrap();
    }

    #[test]
    fn alternate_screen_supported() {
        let _ = TerminalContext::new(
            WindowSetup::default()
                .show_cursor(true)
                .capture_keyboard(false),
        )
        .unwrap();
    }

    #[test]
    fn hide_cursor_supported() {
        let _ = TerminalContext::new(
            WindowSetup::default()
                .show_cursor(false)
                .alternate_screen(false)
                .capture_keyboard(false),
        )
        .unwrap();
    }
}
