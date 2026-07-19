use crate::data::Color;
use crate::ui::util::color_util::color_to_ui_color;
use iced::widget::{Container, column, container, text};
use std::fmt::Display;

#[derive(Debug)]
pub enum DistrictItemType {
    None,
    Station,
    District,
}

impl Display for DistrictItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct DistrictItemDisplay {
    pub name: String,
    pub color: Color,
    pub item_type: DistrictItemType,
}

pub fn district_item_display<'a, Message: 'a>(
    value: DistrictItemDisplay,
) -> Container<'a, Message> {
    let name = text(value.name);
    let color = text(value.color.to_string());
    let background_color = value.color.clone();
    let item_type = text(value.item_type.to_string());

    container(
        column![name, color, item_type]
            .width(150)
            .height(150)
            .padding(25),
    )
    .style(move |_| container::background(color_to_ui_color(background_color)))
}
