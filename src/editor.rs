use std::io::stdout;

use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode, size};
use crossterm::event::{read, KeyEvent, Event, Event::Key, KeyModifiers, KeyCode::Char};
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color, Attribute};
use crossterm::cursor::MoveTo;
use crossterm::execute;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Size {
    width: usize,
    height: usize
}

pub struct Position {
    x: usize,
    y: usize
}

pub struct Terminal {
    size: Size
}

impl Terminal {
    pub fn default() -> Self {
        Self { size: Self::size().unwrap() }
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok(Size { width: cols as usize, height: rows as usize })
    }
}

/// Define an enum for themes
enum Theme {
    Maroon,
    Skylight,
    Electro,
    Custom(Color, Color)
}

impl Theme {
    /// Get the static foreground and background colors for each theme
    fn colors(&self) -> (Color, Color) {
        match self {
            Theme::Maroon => (Color::Red, Color::Grey),
            Theme::Skylight => (Color::Blue, Color::White),
            Theme::Electro => (Color::Yellow, Color::Black),
            Theme::Custom(fg, bg) => (*fg, *bg),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {

    pub fn default() -> Self {
        Self { should_quit: false, terminal: Terminal::default() }
    }

    pub fn run(&mut self) {
        self.initialize().unwrap();
        let result = self.repl();
        self.terminate().unwrap();
        result.unwrap();
    }

    fn initialize(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        self.draw_rows()?;
        self.draw_welcome_message()
    }

    fn terminate(&self) -> Result<(), std::io::Error> {
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
        let Size { width: width, height: height } = self.terminal.size;
        Self::move_cursor_to(0,0);
        for i in 0..=height {
            Self::print("~".to_string(), Theme::Maroon);
            Self::move_cursor_to(0, i as usize)?;
        }
        Self::move_cursor_to(0,0)?;
        Ok(())
    }

    fn draw_welcome_message(&self) -> Result<(), std::io::Error> {
        let Size { width: width, height: height } = self.terminal.size;
        let message = format!("{NAME} - {VERSION}"); // rim - version 0.1
        let y = height as usize / 3;
        let x = (width as usize / 2) - message.len() / 2;
        Self::move_cursor_to(x.try_into().unwrap(), y.try_into().unwrap())?;
        Self::print(message, Theme::Maroon)?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }




}
