use power_switch::power_switch::PowerSwitch;
use smart_house::smart_house::{DeviceInfoProvider, SmartHouse};
use thermometer::thermometer::Thermometer;

const REPORT: &str = r#"Power Switch (state: Off, description: "Bathroom", power consumption: 0)
Thermometer (temperature: 0)
Power Switch (state: Off, description: "Dinning room", power consumption: 0)
Thermometer (temperature: 0)
"#;

struct MyDeviceInfoProvider {
    switch1: PowerSwitch,
    switch2: PowerSwitch,
    thermometer1: Thermometer,
    thermometer2: Thermometer,
}

impl DeviceInfoProvider for MyDeviceInfoProvider {
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

#[tokio::test(flavor = "multi_thread")]
async fn test_report() {
    let dinning_power_switch = PowerSwitch::new("Dinning room");
    let bathroom_power_switch = PowerSwitch::new("Bathroom");
    let dinning_thermometer = Thermometer::new("127.0.0.1:6876", "127.0.0.1:6877")
        .await
        .unwrap();
    let bathroom_thermometer = Thermometer::new("127.0.0.1:7888", "127.0.0.1:7889")
        .await
        .unwrap();

    let smart_house = SmartHouse::generate();

    let info_provider = MyDeviceInfoProvider {
        switch1: dinning_power_switch,
        switch2: bathroom_power_switch,
        thermometer1: dinning_thermometer,
        thermometer2: bathroom_thermometer,
    };

    let report = smart_house.create_report(&info_provider).unwrap();

    assert_eq!(report, REPORT);
}
