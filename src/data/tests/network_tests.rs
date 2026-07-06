use crate::data::{Network, Station, StationColor};

#[test]
pub fn new_creates_all_empty_stations() {
    // Act
    const NETWORK: Network = Network::new();

    // Assert
    for i in 0..StationColor::VARIANTS.len() {
        let colour = StationColor::VARIANTS[i];

        let result = NETWORK.get_station(colour);

        assert!(result.is_none());
    }
}

#[test]
pub fn get_existing_station_is_returned() {
    // Arrange
    let mut network = Network::new();
    let expected_name = "Test Station";
    let expected_colour = StationColor::Blue;
    let station = Station::new(expected_name, expected_colour);
    network.add_station(station);

    // Act
    let result = network.get_station(expected_colour);

    // Assert
    assert!(result.is_some());
    assert_eq!(result.unwrap(), station);
}

#[test]
pub fn get_no_station_returns_none() {
    // Arrange
    let network = Network::new();

    // Act
    let result = network.get_station(StationColor::Cyan);

    // Assert
    assert!(result.is_none());
}

#[test]
pub fn add_station_no_existing_station_adds_and_returns_true() {
    // Arrange
    let mut network = Network::new();
    let expected_name = "Test Station";
    let expected_colour = StationColor::Blue;
    let station = Station::new(expected_name, expected_colour);

    // Act
    let result = network.add_station(station);

    // Assert
    assert!(result);
    let added_station = network.get_station(expected_colour);
    assert!(added_station.is_some());
    assert_eq!(added_station.unwrap(), station);
}

#[test]
pub fn add_station_existing_station_does_not_add_and_returns_false() {
    // Arrange
    let mut network = Network::new();
    let expected_colour = StationColor::Blue;
    let expected_name1 = "Test Station 1";
    let expected_name2 = "Test Station 1";
    let station1 = Station::new(expected_name1, expected_colour);
    let station2 = Station::new(expected_name2, expected_colour);
    network.add_station(station1);

    // Act
    let result = network.add_station(station2);

    // Assert
    assert!(!result);
    let existing_station = network.get_station(expected_colour).unwrap();
    assert_eq!(existing_station, station1);
    assert_eq!(existing_station, station2);
}
