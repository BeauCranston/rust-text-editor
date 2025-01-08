use crossterm::cursor::{self, Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

#[derive(Clone, Copy)]
pub struct TerminalSize {
    pub width: usize,
    pub height: usize,
}
#[derive(Clone, Copy, Default)]
pub struct TerminalPosition {
    pub col: usize,
    pub row: usize,
}

pub struct Terminal;

#[allow(dead_code)]
impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
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
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(pos: TerminalPosition) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }
    pub fn move_to_start() -> Result<(), Error> {
        Self::move_cursor_to(TerminalPosition { col: 0, row: 0 })
    }
    pub fn size() -> Result<TerminalSize, Error> {
        let size = size().unwrap();
        Ok(TerminalSize {
            width: size.0 as usize,
            height: size.1 as usize,
        })
    }
    pub fn print(text: &str) -> Result<(), Error> {
        Self::queue_command(Print(text))?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn current_position() -> Result<TerminalPosition, Error> {
        let position = cursor::position().unwrap();
        Ok(TerminalPosition {
            col: position.0 as usize,
            row: position.1 as usize,
        })
    }
}
