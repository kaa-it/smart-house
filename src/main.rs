use smart_house::power_switch::PowerSwitch;
use smart_house::thermometer::Thermometer;

fn main() {
    let power_switch = PowerSwitch::new("Столовая");
    let thermometer = Thermometer::default();

    println!("Розетка: {:?}", power_switch);
    println!("Термометр: {:?}", thermometer);
}
