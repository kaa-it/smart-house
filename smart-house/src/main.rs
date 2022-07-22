use power_switch::power_switch::PowerSwitch;
use smart_house::errors;
use smart_house::smart_house::{DeviceInfoProvider, SmartHouse};
use thermometer::thermometer::Thermometer;

struct OwningDeviceInfoProvider {
    switch1: PowerSwitch,
    switch2: PowerSwitch,
    thermometer1: Thermometer,
    thermometer2: Thermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn report(&self, room_name: &str, device_name: &str) -> Option<String> {
        match (room_name, device_name) {
            ("Dinning room", "therm1") => Some(format!("{}", self.thermometer1)),
            ("Dinning room", "switch1") => Some(format!("{}", self.switch1)),
            ("Bathroom", "therm2") => Some(format!("{}", self.thermometer2)),
            ("Bathroom", "switch1") => Some(format!("{}", self.switch2)),
            (_, _) => None,
        }
    }
}

struct BorrowingDeviceInfoProvider<'a> {
    switch1: &'a PowerSwitch,
    switch2: &'a PowerSwitch,
    thermometer1: &'a Thermometer,
    thermometer2: &'a Thermometer,
}

impl<'a> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a> {
    fn report(&self, room_name: &str, device_name: &str) -> Option<String> {
        match (room_name, device_name) {
            ("Dinning room", "therm1") => Some(format!("{}", self.thermometer1)),
            ("Dinning room", "switch1") => Some(format!("{}", self.switch1)),
            ("Bathroom", "therm2") => Some(format!("{}", self.thermometer2)),
            ("Bathroom", "switch1") => Some(format!("{}", self.switch2)),
            (_, _) => None,
        }
    }
}

#[tokio::main]
async fn main() -> errors::Result<()> {
    let dinning_power_switch = PowerSwitch::new("Dinning room");
    let bathroom_power_switch = PowerSwitch::new("Bathroom");
    let dinning_thermometer = Thermometer::new("127.0.0.1:6876", "127.0.0.1:6877")
        .await
        .unwrap();
    let bathroom_thermometer = Thermometer::new("127.0.0.1:7888", "127.0.0.1:7889")
        .await
        .unwrap();
    let dinning_thermometer2 = Thermometer::new("127.0.0.1:6878", "127.0.0.1:6879")
        .await
        .unwrap();
    let bathroom_thermometer2 = Thermometer::new("127.0.0.1:7889", "127.0.0.1:7890")
        .await
        .unwrap();

    let smart_house = SmartHouse::generate();

    let info_provider1 = OwningDeviceInfoProvider {
        switch1: dinning_power_switch.clone(),
        switch2: bathroom_power_switch.clone(),
        thermometer1: dinning_thermometer,
        thermometer2: bathroom_thermometer,
    };

    let report1 = smart_house.create_report(&info_provider1)?;

    let info_provider2 = BorrowingDeviceInfoProvider {
        switch1: &dinning_power_switch,
        switch2: &bathroom_power_switch,
        thermometer1: &dinning_thermometer2,
        thermometer2: &bathroom_thermometer2,
    };

    let report2 = smart_house.create_report(&info_provider2)?;

    println!("Report #1: \n{}", report1);
    println!("Report #2: \n{}", report2);

    Ok(())
}
