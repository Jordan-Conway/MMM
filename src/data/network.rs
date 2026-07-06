use crate::data::{Station, StationColor};

#[derive(Debug, Clone, Copy)]
pub struct Network {
    stations : [[Option<Station>; 4]; 4]
}

impl Network {
    pub const fn new() -> Network {
        return Network { stations: [[None; 4]; 4] }
    }

    pub fn add_station(&mut self, station: Station) -> bool {
        let (x, y) = map_color_to_position(station.colour);

        let existing = self.stations[x][y];

        return match existing {
            Some(_) => false,
            None => {
                self.stations[x][y] = Some(station);
                true
            }
        }
    }

    pub fn get_station(&self, colour: StationColor) -> Option<Station> {
        let (x, y) = map_color_to_position(colour);
        return self.stations[x][y];
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

fn map_color_to_position(colour: StationColor) -> (usize, usize) {
    return match colour {
        StationColor::Red => (0, 0),
        StationColor::Orange => (0, 1),
        StationColor::Yellow => (0, 2),
        StationColor::Lime => (0, 3),
        StationColor::Green => (1, 0),
        StationColor::Cyan => (1, 1),
        StationColor::LightBlue => (1, 2),
        StationColor::Blue => (1, 3),
        StationColor::Pink => (2, 0),
        StationColor::Magenta => (2, 1),
        StationColor::Purple => (2, 2),
        StationColor::Brown => (2, 3),
        StationColor::White => (3, 0),
        StationColor::LightGray => (3, 1),
        StationColor::Gray => (3, 2),
        StationColor::Black => (3, 3)
    }
}