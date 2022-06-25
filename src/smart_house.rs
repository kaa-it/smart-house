//! Module describes smart house

use std::collections::BTreeMap;

use crate::errors::{self, Error::DeviceNotFoundError};

use self::room::{DeviceList, Room};

mod room;

/// Describes list of rooms in the smart house
pub type RoomList = BTreeMap<String, Room>;
/// Describes smart house
pub struct SmartHouse {
    name: String,
    rooms: RoomList,
}

impl SmartHouse {
    /// Creates new empty house with given name
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rooms: BTreeMap::new(),
        }
    }

    /// Generates small house
    pub fn generate() -> Self {
        let mut house = Self::new("Our house");

        let mut dinning_room = Room::new("Dinning room");
        dinning_room.add_device("therm1");
        dinning_room.add_device("switch1");

        let mut bathroom = Room::new("Bathroom");
        bathroom.add_device("therm2");
        bathroom.add_device("switch1");

        house.add_room(dinning_room);
        house.add_room(bathroom);

        house
    }

    /// Returns name of the smart house
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Sets name to the smart house
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    /// Returns list of rooms for the smart house
    pub fn get_rooms(&self) -> &RoomList {
        &self.rooms
    }

    /// Adds room to the smart house.
    /// Returns `true` if room was added,
    /// returns `false` otherwise.
    pub fn add_room(&mut self, room: Room) -> bool {
        if self.rooms.contains_key(room.get_name()) {
            return false;
        }

        _ = self.rooms.insert(room.get_name().to_owned(), room);

        true
    }

    /// Removes room from the smart house.
    /// Returns `true` if room was removed,
    /// returns `false` otherwise.
    pub fn remove_room(&mut self, room_name: &str) -> bool {
        if !self.rooms.contains_key(room_name) {
            return false;
        }

        _ = self.rooms.remove(room_name);

        true
    }

    /// Returns names of devices for the room of smart house by room's name
    pub fn devices(&self, room_name: &str) -> Option<&DeviceList> {
        self.rooms
            .get(&room_name.to_owned())
            .map(|r| r.get_devices())
    }

    /// Returns report about devices of the smart house
    ///
    /// `provider` - provider of info about devices
    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> errors::Result<String> {
        let mut report = String::new();

        for r in self.rooms.values() {
            for d in r.get_devices() {
                let device_report =
                    provider
                        .report(r.get_name(), d)
                        .ok_or_else(|| DeviceNotFoundError {
                            device_name: d.clone(),
                            room_name: r.get_name().to_owned(),
                        })?;

                report.push_str(&device_report);
                report.push('\n');
            }
        }

        Ok(report)
    }
}

/// Describes contract for provider of info about devices
pub trait DeviceInfoProvider {
    /// Returns description of device state by room name and device name
    /// If given device is not found, returns None
    fn report(&self, room_name: &str, device_name: &str) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_devices_by_room_name() {
        let smart_house = SmartHouse::generate();

        let devices = smart_house.devices("Bathroom").unwrap();

        assert!(devices.contains("therm2"));
        assert!(devices.contains("switch1"));
    }

    #[test]
    fn test_get_devices_panics_if_room_name_not_found() {
        let smart_house = SmartHouse::generate();

        let devices = smart_house.devices("Kitchen");

        assert_eq!(devices, None);
    }
}
