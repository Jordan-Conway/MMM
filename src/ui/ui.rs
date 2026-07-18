use crate::data::{Color, District, DistrictItem, Station};
use crate::ui::components::district_grid::district_grid;
use iced::Element;
use iced::widget::{button, column};

#[derive(Debug, Clone)]
enum Message {
    AddItem(DistrictItem),
}

fn view(district: &District) -> Element<'_, Message> {
    let add_station_button = button("Add button").on_press(Message::AddItem(
        DistrictItem::Station(Station::new("My New Station", Color::Cyan)),
    ));

    column![district_grid(&district), add_station_button].into()
}

fn update(district: &mut District, message: Message) {
    match message {
        Message::AddItem(item) => {
            district.add_item(item);
        }
    }
}

pub fn run() -> iced::Result {
    iced::run(update, view)
}
