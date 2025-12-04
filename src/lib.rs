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

#[cfg(test)]
mod tests {
    use super::*;
}
