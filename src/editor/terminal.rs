use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::fmt::Display;
use std::io::{stdout, Error, Write};

#[derive(Clone, Copy)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}
#[derive(Clone, Copy)]
pub struct TerminalPosition {
    pub x: u16,
    pub y: u16,
}
pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_to_start()?;
        Self::execute()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(pos: TerminalPosition) -> Result<(), std::io::Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Show)
    }
    pub fn move_to_start() -> Result<(), std::io::Error> {
        Self::move_cursor_to(TerminalPosition { x: 0, y: 0 })
    }
    pub fn size() -> Result<TerminalSize, std::io::Error> {
        let size = size().unwrap();
        Ok(TerminalSize {
            width: size.0,
            height: size.1,
        })
    }
    pub fn print<T: Display>(text: T) -> Result<(), std::io::Error> {
        Self::queue_command(Print(text))?;
        Ok(())
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
}
