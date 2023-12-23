use std::io::{self, Write};

use nix::sys::termios::{self, SetArg, Termios};

pub struct RawTerminal {
    original_termios: Termios,
}

impl RawTerminal {
    pub fn new() -> Self {
        let original_termios = termios::tcgetattr(io::stdin()).unwrap();

        Self::make_terminal_raw(&original_termios);
        Self::hide_cursor();
        Self::clear_screen();

        Self { original_termios }
    }

    fn make_terminal_raw(termios: &Termios) {
        let mut raw_termios = termios.clone();
        termios::cfmakeraw(&mut raw_termios);
        termios::tcsetattr(io::stdin(), SetArg::TCSANOW, &raw_termios).unwrap();
    }

    fn hide_cursor() {
        io::stdout().write_all(b"\x1B[?25l").unwrap();
    }

    fn show_cursor() {
        io::stdout().write_all(b"\x1B[?25h").unwrap();
    }

    fn clear_screen() {
        io::stdout().write_all(b"\x1B[2J").unwrap();
    }

    fn restore_original_termios(&self) {
        termios::tcsetattr(io::stdin(), SetArg::TCSANOW, &self.original_termios).unwrap();
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        self.restore_original_termios();
        Self::show_cursor();
    }
}
