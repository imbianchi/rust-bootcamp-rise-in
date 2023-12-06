use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::fs;

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    is_first_time: bool,
}

impl Terminal {
    pub fn run() -> Result<(), std::io::Error> {
        let size = termion::terminal_size()?;

        if !fs::metadata("./.config").is_ok() {
            // create config file
            // run welcome screen
            // redirects to login screen
            // return Ok
            return Ok(())
        }

        // check if is already logged
        // if yes, redirect to dashboard
        // if not, load login page 
        Ok(())
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
}
