//! Module describes smart house

/// Describes list of devices in the room
pub type DeviceList<'a> = [&'a str; 2];

/// Describes list of rooms in the smart house
pub type RoomList<'a> = [Room<'a>; 2];

/// Describes room of the smart house
#[derive(Debug)]
pub struct Room<'a> {
    name: String,
    devices: DeviceList<'a>,
}

/// Describes smart house
pub struct SmartHouse<'a> {
    _name: String,
    rooms: RoomList<'a>,
}

impl<'a> Default for SmartHouse<'a> {
    /// Create default smart house
    fn default() -> Self {
        Self {
            _name: String::from("Our house"),
            rooms: [
                Room {
                    name: String::from("Dinning room"),
                    devices: ["therm1", "switch1"],
                },
                Room {
                    name: String::from("Bathroom"),
                    devices: ["therm2", "switch1"],
                },
            ],
        }
    }
}

impl<'a> SmartHouse<'a> {
    /// Returns list of rooms for the smart house
    pub fn get_rooms(&self) -> &RoomList {
        &self.rooms
    }

    /// Returns names of devices for the room of smart house by room's name
    pub fn devices(&self, room_name: &str) -> DeviceList {
        for r in &self.rooms {
            if r.name == room_name {
                return r.devices;
            }
        }

        panic!("Room not found");
    }

    /// Returns report about devices of the smart house
    ///
    /// `provider` - provider of info about devices
    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = String::new();

        for r in &self.rooms {
            for d in &r.devices {
                report.push_str(&provider.report(&r.name, d));
                report.push('\n');
            }
        }

        report
    }
}

/// Describes contract for provider of info about devices
pub trait DeviceInfoProvider {
    /// Returns description of device state by room name and device name
    fn report(&self, room_name: &str, device_name: &str) -> String;
}
