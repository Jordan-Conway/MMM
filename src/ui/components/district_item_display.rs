use crate::data::Color;
use iced::widget::{Column, column, text};
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

pub fn district_item_display<'a, Message>(value: DistrictItemDisplay) -> Column<'a, Message> {
    let name = text(value.name);
    let color = text(value.color.to_string());
    let item_type = text(value.item_type.to_string());

    column![name, color, item_type].padding(25)
}
