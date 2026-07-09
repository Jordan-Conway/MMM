use crate::data::{Color, District, DistrictItem, Station, district::DistrictType};

#[test]
pub fn district_item_get_color_gets_station_color() {
    // Arrange
    let expected = Color::Black;
    let district_item = DistrictItem::Station(Station::new("Test Station", expected));

    // Act
    let actual = district_item.get_color();

    // Assert
    assert_eq!(expected, actual);
}

#[test]
pub fn district_item_get_color_gets_district_color() {
    // Arrange
    let expected = Color::Black;
    let district_item = DistrictItem::District(District::new(expected, DistrictType::Network));

    // Act
    let actual = district_item.get_color();

    // Assert
    assert_eq!(expected, actual);
}

#[test]
pub fn new_creates_all_empty_items() {
    // Act
    let district = create_default_district(DistrictType::Network);

    // Assert
    for i in 0..Color::VARIANTS.len() {
        let colour = Color::VARIANTS[i];

        let result = district.get_item(colour);

        assert!(result.is_none());
    }
}

#[test]
pub fn get_item_existing_item_is_returned() {
    // Arrange
    let mut network = create_default_district(DistrictType::Network);
    let expected_name = "Test Station";
    let expected_colour = Color::Blue;
    let item = DistrictItem::Station(Station::new(expected_name, expected_colour));
    network.add_item(item.clone());

    // Act
    let result = network.get_item(expected_colour);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap(), item);
}

#[test]
pub fn get_no_station_returns_none() {
    // Arrange
    let network = create_default_district(DistrictType::Region);

    // Act
    let result = network.get_item(Color::Cyan);

    // Assert
    assert!(result.is_none());
}

#[test]
pub fn add_item_no_existing_item_adds_and_returns_true() {
    // Arrange
    let mut network = create_default_district(DistrictType::Network);
    let expected_name = "Test Station";
    let expected_colour = Color::Blue;
    let station = Station::new(expected_name, expected_colour);
    let item = DistrictItem::Station(station.clone());

    // Act
    let result = network.add_item(item);

    // Assert
    assert!(result);
    let added_item = network.get_item(expected_colour);
    assert!(added_item.is_some());
    let added_station = match added_item {
        Some(item) => match item {
            DistrictItem::District(_) => panic!("Expected station, found district"),
            DistrictItem::Station(station) => station,
        },
        None => panic!("Expected some, found none"),
    };
    assert_eq!(station, added_station);
}

#[test]
pub fn add_item_existing_item_does_not_add_and_returns_false() {
    // Arrange
    let mut network = create_default_district(DistrictType::Network);
    let expected_colour = Color::Blue;
    let expected_name1 = "Test Station 1";
    let expected_name2 = "Test Station 2";
    let station1 = Station::new(expected_name1, expected_colour);
    let station2 = Station::new(expected_name2, expected_colour);
    network.add_item(DistrictItem::Station(station1));

    // Act
    let result = network.add_item(DistrictItem::Station(station2));

    // Assert
    assert!(!result);
    let existing_item = network.get_item(expected_colour).unwrap();
    match existing_item {
        DistrictItem::District(_) => panic!("Expected station, found district"),
        DistrictItem::Station(existing_station) => {
            assert_eq!(station1, existing_station);
        }
    };
}

#[test]
pub fn add_wrong_district_item_variant_does_not_add_and_returns_false() {
    // Arrange
    let mut district = create_default_district(DistrictType::Network);
    let item_color = Color::Black;
    let district_item = DistrictItem::District(District::new(item_color, DistrictType::Network));

    // Act
    let result = district.add_item(district_item);

    // Assert
    assert!(!result);
    let existing = district.get_item(item_color);
    assert!(existing.is_none());
}

fn create_default_district(district_type: DistrictType) -> District {
    District::new(Color::Black, district_type)
}
