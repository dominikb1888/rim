use std::io::stdout;

use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::execute;

pub struct Editor { }


impl Editor {

    pub fn default() -> Self {
        Self { }
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        Self::clear_screen();
        Self::repl();
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn repl() {
        loop {
            match read() {
                Ok(Key(event)) => {
                    match event.code {
                        Char('q') => {
                            disable_raw_mode().unwrap();
                            break;
                        }
                        Char(c) => {
                            println!("{c}");
                        }
                        _ => { todo!() }
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => todo!(),
            }
        }
    }
}
