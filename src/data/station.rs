use crate::data::station_colour::StationColor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Station {
    pub name: &'static str,
    pub colour: StationColor,
    pub x: i32,
    pub y: i32
}

impl Station {
    pub const fn new(name: &'static str, colour: StationColor) -> Station {
        return Station { name: name, colour: colour, x: 0, y: 0 }
    }
}

impl Default for Station {
    fn default() -> Self {
        Self::new("", StationColor::Red)
    }
}