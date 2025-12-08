use crossterm::{cursor, queue, style, terminal};
use std::io::{self, BufWriter, Stdout, Write};

pub struct WindowSetup {
    capture_keyboard: bool,
    alternate_screen: bool,
    hide_cursor: bool,
}

impl WindowSetup {
    pub fn default() -> Self {
        WindowSetup {
            capture_keyboard: true,
            alternate_screen: true,
            hide_cursor: true,
        }
    }

    pub fn hide_cursor(mut self, hide: bool) -> Self {
        self.hide_cursor = hide;
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
    hide_cursor: bool,
    writer: BufWriter<Stdout>,
    size: (u16, u16),
}

impl TerminalContext {
    /// Creates a new context for the terminal
    /// with the settings specified in setup
    pub fn new(setup: WindowSetup) -> Result<Self, io::Error> {
        let mut ctx = Self {
            raw_mode: false,
            alternate_screen: false,
            hide_cursor: false,
            writer: BufWriter::new(io::stdout()),
            size: (0, 0),
        };
        ctx.size = terminal::size()?;
        if setup.capture_keyboard {
            terminal::enable_raw_mode()?;
            ctx.raw_mode = true;
        }
        if setup.alternate_screen {
            queue!(ctx.writer, terminal::EnterAlternateScreen)?;
            ctx.alternate_screen = true;
        }
        if setup.hide_cursor {
            queue!(ctx.writer, cursor::Hide)?;
            ctx.hide_cursor = true;
        }
        ctx.writer.flush()?;
        Ok(ctx)
    }
}

impl Drop for TerminalContext {
    fn drop(&mut self) {
        if self.raw_mode {
            let _ = terminal::disable_raw_mode();
        }
        if self.alternate_screen {
            let _ = queue!(self.writer, terminal::LeaveAlternateScreen);
        }
        if self.hide_cursor {
            let _ = queue!(self.writer, cursor::Show);
        }
    }
}

pub struct Canvas {
    buffer: Vec<bool>,
    change_buffer: Vec<bool>,
    pub height: usize,
    pub width: usize,
}

impl Canvas {
    pub fn new(ctx: &TerminalContext) -> Self {
        let (columns, rows) = ctx.size;
        let (width, height) = (columns as usize, rows as usize * 2);
        let buffer: Vec<bool> = vec![false; width * height];
        let change_buffer: Vec<bool> = vec![true; width * rows as usize];
        Self {
            buffer,
            change_buffer,
            width,
            height,
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize) -> bool {
        let index = self.width * y + x;
        if let Some(pixel) = self.buffer.get_mut(index) {
            *pixel = true;
            // Update the corresponding char position in change buffer
            if let Some(change) = self.change_buffer.get_mut(self.width * (y / 2) + x) {
                *change = true;
            }
        } else {
            return false;
        };
        true
    }

    fn print_pixel(
        &self,
        ctx: &mut TerminalContext,
        col: usize,
        row: usize,
    ) -> Result<(), io::Error> {
        // Skip printing if no change
        let char_index = self.width * row + col;
        if let Some(change) = self.change_buffer.get(char_index) {
            if *change == false {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        // Set colors and print
        let top_index = self.width * (row * 2) + col;
        if let Some(top_pixel) = self.buffer.get(top_index) {
            if *top_pixel {
                queue!(ctx.writer, style::SetBackgroundColor(style::Color::White))?;
            } else {
                queue!(ctx.writer, style::SetBackgroundColor(style::Color::Black))?;
            }
        };
        let bottom_index = self.width * (row * 2 + 1) + col;
        if let Some(bottom_pixel) = self.buffer.get(bottom_index) {
            if *bottom_pixel {
                queue!(ctx.writer, style::SetForegroundColor(style::Color::White))?;
            } else {
                queue!(ctx.writer, style::SetForegroundColor(style::Color::Black))?;
            }
        };
        queue!(
            ctx.writer,
            cursor::MoveTo(col as u16, row as u16),
            style::Print("▄")
        )?;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut TerminalContext) -> Result<(), io::Error> {
        for row in 0..(self.height / 2) {
            for col in 0..self.width {
                let _ = self.print_pixel(ctx, col, row);
            }
        }
        queue!(ctx.writer, style::ResetColor)?;
        ctx.writer.flush()?;
        self.change_buffer = vec![false; (self.height / 2) * self.width];
        Ok(())
    }

    pub fn clear(&mut self) -> () {
        self.buffer = vec![false; self.width * self.height];
        self.change_buffer = vec![true; self.width * self.height / 2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_mode_supported() {
        let _ = TerminalContext::new(
            WindowSetup::default()
                .hide_cursor(false)
                .alternate_screen(false),
        )
        .unwrap();
    }

    #[test]
    fn alternate_screen_supported() {
        let _ = TerminalContext::new(
            WindowSetup::default()
                .hide_cursor(false)
                .capture_keyboard(false),
        )
        .unwrap();
    }

    #[test]
    fn hide_cursor_supported() {
        let _ = TerminalContext::new(
            WindowSetup::default()
                .alternate_screen(false)
                .capture_keyboard(false),
        )
        .unwrap();
    }
}
