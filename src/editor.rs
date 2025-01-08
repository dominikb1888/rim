use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {

}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {

    #[must_use]
    pub fn new() -> Self {
        Editor{}
    }

    #[allow(clippy::panic)]
    pub fn run(&self){
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                },
                Err(err) => println!("Error: {err}"),
                _ => ()
            }
        }
    disable_raw_mode().unwrap();
    }
}

