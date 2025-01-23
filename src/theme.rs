use crossterm::style::{Color};

pub enum Theme {
    Maroon,
    Skylight,
    Electro,
    Custom(Color, Color)
}

impl Theme {
    pub fn colors(&self) -> (Color, Color) {
        match self {
            Theme::Maroon => (Color::Red, Color::Grey),
            Theme::Skylight => (Color::Blue, Color::White),
            Theme::Electro => (Color::Yellow, Color::Black),
            Theme::Custom(fg, bg) => (*fg, *bg),
        }
    }
}
