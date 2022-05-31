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
