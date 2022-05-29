//! Module describes thermometer device for smart house

/// Describes smart thermometer
#[derive(Debug)]
pub struct Thermometer {
    temperature: f64,
}

impl Default for Thermometer {
    fn default() -> Self {
        Self { temperature: 0.0 }
    }
}

impl Thermometer {
    /// Returns current temperature of the thermometer
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}
