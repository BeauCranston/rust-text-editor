use core::cmp::min;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::{self},
    KeyEvent, KeyEventKind, KeyModifiers,
};
use std::io::Error;
mod terminal;
mod view;
use terminal::{Terminal, TerminalPosition, TerminalSize};
use view::View;

#[derive(Default)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
}

impl Editor {
    ///Runs the editor and terminates when finished
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    ///reads input and responds to user input
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            let _ = self.evaluate_event(&event);
        }
        Ok(())
    }
    fn move_caret(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let TerminalSize { width, height } = Terminal::size()?;
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            KeyCode::Tab => x = min(width.saturating_sub(1), x.saturating_add(4)),
            KeyCode::BackTab => x = x.saturating_sub(4),
            KeyCode::PageUp => y = y.saturating_sub(height.saturating_sub(1)),
            KeyCode::PageDown => y = y.saturating_add(height.saturating_sub(1)),
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }
    ///based on the input the terminal will perform some action
    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Tab
                | KeyCode::BackTab => {
                    self.move_caret(*code)?;
                }

                _ => (),
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(TerminalPosition::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(TerminalPosition {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
