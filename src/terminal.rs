use crate::Position;
use std::io::{self, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    /// Set up a terminal with the appropriate size
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    /// Returns a reference to the size of this [`Terminal`].
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Clear  the terminal screen
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    /// Set cursor position
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// Flush stdout
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    /// Read a key from the standard input
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    /// Hides cursor from terminal
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    /// Displays cursor on terminal (after hiding)
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    /// Clear current line
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
