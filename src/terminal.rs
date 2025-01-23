use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use std::io::stdout;

pub struct Size {
   pub width: usize,
   pub height: usize
}

pub struct Position {
    pub x: usize,
    pub y: usize
}

pub struct Terminal {
    pub size: Size
}

impl Terminal {
    pub fn default() -> Self {
        Self { size: Self::size().unwrap() }
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok(Size { width: cols as usize, height: rows as usize })
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

}


