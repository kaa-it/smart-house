use std::collections::BTreeSet;

/// Describes list of devices in the room
pub type DeviceList = BTreeSet<String>;

/// Describes room of the smart house
#[derive(Debug)]
pub struct Room {
    name: String,
    devices: DeviceList,
}

impl Room {
    /// Creates new empty room with given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            devices: BTreeSet::new(),
        }
    }

    /// Returns name of the room
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns list of devices in the room
    pub fn get_devices(&self) -> &DeviceList {
        &self.devices
    }

    /// Adds device with given name to the room.
    /// Returns `true` if device was added,
    /// returns `false` otherwise.
    pub fn add_device(&mut self, device_name: &str) -> bool {
        self.devices.insert(device_name.to_owned())
    }

    /// Removes device with given name from the room.
    /// Returns `true` if device was removed,
    /// returns `false` otherwise.
    pub fn remove_device(&mut self, device_name: &str) -> bool {
        self.devices.remove(device_name)
    }
}

mod tests {

    #[test]
    fn test_add_device() {
        let mut room = super::Room::new("Bathroom");

        let added = room.add_device("switch1");

        assert!(added);
        assert_eq!(room.devices.len(), 1);
        assert!(room.devices.contains("switch1"));
    }

    #[test]
    fn test_add_already_existed() {
        let mut room = super::Room::new("Bathroom");

        _ = room.add_device("switch1");
        let added = room.add_device("switch1");

        assert!(!added);
        assert_eq!(room.devices.len(), 1);
        assert!(room.devices.contains("switch1"));
    }

    #[test]
    fn test_remove_existed_device() {
        let mut room = super::Room::new("Bathroom");

        _ = room.add_device("switch1");
        let removed = room.remove_device("switch1");

        assert!(removed);
        assert_eq!(room.devices.len(), 0);
    }

    #[test]
    fn test_remove_not_existed_device() {
        let mut room = super::Room::new("Bathroom");

        _ = room.add_device("switch1");
        let removed = room.remove_device("switch2");

        assert!(!removed);
        assert_eq!(room.devices.len(), 1);
        assert!(room.devices.contains("switch1"));
        assert!(!room.devices.contains("switch2"));
    }
}
