//! Module describes thermometer device for smart house

/// Describes smart thermometer
#[derive(Debug, Default)]
pub struct Thermometer {
    temperature: f64,
}

impl Thermometer {
    /// Returns current temperature of the thermometer
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}
