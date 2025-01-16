use std::io::stdout;

use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, KeyEvent, Event::Key, KeyModifiers, KeyCode::Char};
use crossterm::execute;

pub struct Editor {
    should_quit: bool,
}


impl Editor {

    pub fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn terminate(&self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        Self::clear_screen()?;
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent { code, modifiers, kind, state }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
