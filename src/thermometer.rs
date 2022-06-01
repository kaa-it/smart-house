//! Module describes thermometer device for smart house

use std::fmt;
/// Describes smart thermometer
#[derive(Debug, Default, Clone)]
pub struct Thermometer {
    temperature: f64,
}

impl Thermometer {
    /// Returns current temperature of the thermometer
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

impl fmt::Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thermometer (temperature: {})", self.temperature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_thermometer() {
        let thermometer = Thermometer::default();

        assert_eq!(thermometer.temperature(), 0.0);
    }

    #[test]
    fn test_display_thermometer() {
        const THERMOMETER_INFO: &str = "Thermometer (temperature: 0)";

        let thermometer = Thermometer::default();

        let thermometer_info = format!("{}", thermometer);

        assert_eq!(thermometer_info, THERMOMETER_INFO);
    }
}
