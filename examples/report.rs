use smart_house::power_switch::PowerSwitch;
use smart_house::smart_house::{DeviceInfoProvider, SmartHouse};
use smart_house::thermometer::Thermometer;

struct MyDeviceInfoProvider {
    switch1: PowerSwitch,
    switch2: PowerSwitch,
    thermometer1: Thermometer,
    thermometer2: Thermometer,
}

impl DeviceInfoProvider for MyDeviceInfoProvider {
    fn report(&self, room_name: &str, device_name: &str) -> String {
        match (room_name, device_name) {
            ("Dinning room", "therm1") => format!("{}", self.thermometer1),
            ("Dinning room", "switch1") => format!("{}", self.switch1),
            ("Bathroom", "therm2") => format!("{}", self.thermometer2),
            ("Bathroom", "switch1") => format!("{}", self.switch2),
            (_, _) => format!(
                "Not found device \"{}\" in room \"{}\"",
                device_name, room_name
            ),
        }
    }
}

fn main() {
    let dinning_power_switch = PowerSwitch::new("Dinning room");
    let bathroom_power_switch = PowerSwitch::new("Bathroom");
    let dinning_thermometer = Thermometer::default();
    let bathroom_thermometer = Thermometer::default();

    let smart_house = SmartHouse::default();

    let info_provider = MyDeviceInfoProvider {
        switch1: dinning_power_switch,
        switch2: bathroom_power_switch,
        thermometer1: dinning_thermometer,
        thermometer2: bathroom_thermometer,
    };

    let report = smart_house.create_report(&info_provider);

    println!("Report: \n{}", report);
}
