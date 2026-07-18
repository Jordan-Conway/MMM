use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    LightBlue,
    Blue,
    Pink,
    Magenta,
    Purple,
    Brown,
    White,
    LightGray,
    Gray,
    Black,
}

impl Color {
    pub const VARIANTS: [Color; 16] = [
        Color::Red,
        Color::Orange,
        Color::Yellow,
        Color::Lime,
        Color::Green,
        Color::Cyan,
        Color::LightBlue,
        Color::Blue,
        Color::Pink,
        Color::Magenta,
        Color::Purple,
        Color::Brown,
        Color::White,
        Color::LightGray,
        Color::Gray,
        Color::Black,
    ];
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
