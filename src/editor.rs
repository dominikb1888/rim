use std::io::stdout;

// using these imports
use crossterm::event::{read, KeyEvent, Event, Event::Key, KeyModifiers, KeyCode::Char};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor};
use crossterm::cursor::MoveTo;
use crossterm::execute;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::theme::Theme;
use crate::terminal::{Terminal, Size};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {

    pub fn default() -> Self {
        Self { should_quit: false, terminal: Terminal::default() }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }



    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        self.draw_rows()?;
        self.draw_welcome_message()?;

        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Char(c) => println!("{c}"),
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }

    fn print(message: String, theme: Theme) -> Result<(), std::io::Error> {
        let (fg, bg) = theme.colors();
        execute!(stdout(), SetForegroundColor(fg), SetBackgroundColor(bg), Print(message), ResetColor)?;
        Ok(())
    }

    fn move_cursor_to(x: usize, y: usize) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x as u16, y as u16))?;
        Ok(())
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        let Size { width: _, height } = self.terminal.size;
        Self::move_cursor_to(0,0)?;
        for i in 0..=height {
            Self::print("~".to_string(), Theme::Maroon)?;
            Self::move_cursor_to(0, i as usize)?;
        }
        Self::move_cursor_to(0,0)?;
        Ok(())
    }

    fn draw_welcome_message(&self) -> Result<(), std::io::Error> {
        let Size { width, height } = self.terminal.size;
        let message = format!("{NAME} - {VERSION}"); // rim - version 0.1
        let y = height as usize / 3;
        let x = (width as usize / 2) - message.len() / 2;
        Self::move_cursor_to(x.try_into().unwrap(), y.try_into().unwrap())?;
        Self::print(message, Theme::Maroon)?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }




}
