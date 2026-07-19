use crate::{
    data::{Color, District, DistrictItem},
    ui::components::district_item_display::{
        DistrictItemDisplay, DistrictItemType, district_item_display,
    },
};
use iced::widget::{Column, Row};

pub fn district_grid<'a, Message: 'a>(district: &District) -> Column<'a, Message> {
    let mut grid = Column::new();

    for i in 0..4 {
        let mut row = Row::new();
        for j in 0..4 {
            let color = Color::VARIANTS[(i * 4) + j];
            let item = district.get_item(color);
            match item {
                Some(district_item) => match district_item {
                    DistrictItem::District(_) => {
                        let display = DistrictItemDisplay {
                            name: "District".to_string(),
                            color: color,
                            item_type: DistrictItemType::District,
                        };
                        row = row.push(district_item_display(display));
                    }
                    DistrictItem::Station(station) => {
                        let display = DistrictItemDisplay {
                            name: station.name.to_string(),
                            color: color,
                            item_type: DistrictItemType::Station,
                        };
                        row = row.push(district_item_display(display));
                    }
                },
                None => {
                    let display = DistrictItemDisplay {
                        name: "None".to_string(),
                        color: color,
                        item_type: DistrictItemType::None,
                    };
                    row = row.push(district_item_display(display));
                }
            }
        }
        grid = grid.push(row);
    }

    grid
}
