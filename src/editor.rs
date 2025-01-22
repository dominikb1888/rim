use std::io::stdout;

use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size};
use crossterm::event::{read, KeyEvent, Event, Event::Key, KeyModifiers, KeyCode::Char};
use crossterm::cursor::MoveTo;
use crossterm::execute;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}


impl Editor {

    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()?;
        Self::draw_welcome_message()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
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
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let (height, width) = size()?;
        execute!(stdout(), MoveTo(0,0));
        for i in 0..=height {
            print!("~");
            execute!(stdout(), MoveTo(0, i));
        }
        execute!(stdout(), MoveTo(0,0));
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let (width, height) = size()?;
        let message = format!("{NAME} - {VERSION}");
        let length: u16 = message.len().try_into().unwrap();
        let y: u16 = height / 3;
        let x: u16 = width / 2 - length / 2;

        execute!(stdout(), MoveTo(x, y))?;
        for (i, c) in message.chars().enumerate() {
            execute!(stdout(), MoveTo(x + i as u16, y))?;
            print!("{c}");
        }
        execute!(stdout(), MoveTo(0,0))?;
        Ok(())
    }
}
