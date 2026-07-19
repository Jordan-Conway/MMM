use crate::{data::Color, ui::constants::navigation_colors};
use iced::Color as UiColor;

pub fn color_to_ui_color(color: Color) -> UiColor {
    match color {
        Color::Red => navigation_colors::RED,
        Color::Orange => navigation_colors::ORANGE,
        Color::Yellow => navigation_colors::YELLOW,
        Color::Green => navigation_colors::GREEN,
        Color::Lime => navigation_colors::LIME,
        Color::Cyan => navigation_colors::CYAN,
        Color::LightBlue => navigation_colors::LIGHT_BLUE,
        Color::Blue => navigation_colors::BLUE,
        Color::Pink => navigation_colors::PINK,
        Color::Magenta => navigation_colors::MAGENTA,
        Color::Purple => navigation_colors::PURPLE,
        Color::Brown => navigation_colors::BROWN,
        Color::White => navigation_colors::WHITE,
        Color::LightGray => navigation_colors::LIGHT_GRAY,
        Color::Gray => navigation_colors::GRAY,
        Color::Black => navigation_colors::BLACK,
    }
}
