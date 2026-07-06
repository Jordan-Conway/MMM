#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StationColor {
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
    Black
}

impl StationColor {
    pub const VARIANTS: [StationColor; 16] = [
        StationColor::Red,
        StationColor::Orange,
        StationColor::Yellow,
        StationColor::Lime,
        StationColor::Green,
        StationColor::Cyan,
        StationColor::LightBlue,
        StationColor::Blue,
        StationColor::Pink,
        StationColor::Magenta,
        StationColor::Purple,
        StationColor::Brown,
        StationColor::White,
        StationColor::LightGray,
        StationColor::Gray,
        StationColor::Black
    ];
}