use crate::data::{Color, Station};

#[derive(Debug, Clone, PartialEq)]
pub enum DistrictItem {
    District(District),
    Station(Station),
}

impl DistrictItem {
    pub fn get_color(&self) -> Color {
        match self {
            DistrictItem::District(item) => item.color,
            DistrictItem::Station(item) => item.colour,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistrictType {
    Region,  // Regions contain stations
    Network, // Networks contain districts
}

#[derive(Debug, Clone, Copy)]
pub enum DistrictStation {
    Shared(Color),
    Distinct(Station),
}

#[derive(Debug, Clone, PartialEq)]
pub struct District {
    items: Box<[[Option<DistrictItem>; 4]; 4]>,
    pub district_type: DistrictType,
    pub color: Color,
}

impl District {
    pub fn new(color: Color, district_type: DistrictType) -> District {
        return District {
            items: Box::new(std::array::from_fn(|_| std::array::from_fn(|_| None))),
            color: color,
            district_type: district_type,
        };
    }

    pub fn add_item(&mut self, item: DistrictItem) -> bool {
        if !self.can_add_item(&item) {
            false
        } else {
            let item_color = item.get_color();
            let (row, col) = map_color_to_position(item_color);
            return match self.items[row][col] {
                Some(_) => false,
                None => {
                    self.items[row][col] = Some(item);
                    true
                }
            };
        }
    }

    pub fn get_item(&self, color: Color) -> Option<DistrictItem> {
        let (x, y) = map_color_to_position(color);
        self.items[x][y].clone()
    }

    fn can_add_item(&self, item: &DistrictItem) -> bool {
        return match self.district_type {
            DistrictType::Region => match item {
                DistrictItem::District(_) => true,
                DistrictItem::Station(_) => false,
            },
            DistrictType::Network => match item {
                DistrictItem::District(_) => false,
                DistrictItem::Station(_) => true,
            },
        };
    }
}

impl Default for District {
    fn default() -> Self {
        Self {
            items: Default::default(),
            district_type: DistrictType::Network,
            color: Color::Red,
        }
    }
}

fn map_color_to_position(colour: Color) -> (usize, usize) {
    return match colour {
        Color::Red => (0, 0),
        Color::Orange => (0, 1),
        Color::Yellow => (0, 2),
        Color::Lime => (0, 3),
        Color::Green => (1, 0),
        Color::Cyan => (1, 1),
        Color::LightBlue => (1, 2),
        Color::Blue => (1, 3),
        Color::Pink => (2, 0),
        Color::Magenta => (2, 1),
        Color::Purple => (2, 2),
        Color::Brown => (2, 3),
        Color::White => (3, 0),
        Color::LightGray => (3, 1),
        Color::Gray => (3, 2),
        Color::Black => (3, 3),
    };
}
