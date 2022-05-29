//! Module describes power switch device for smart house

/// Describes state of power switch
#[derive(Debug)]
pub enum SwitchState {
    On,
    Off,
}

/// Describes smart power switch
#[derive(Debug)]
pub struct PowerSwitch {
    state: SwitchState,
    description: String,
    power_consumption: f64,
}

impl PowerSwitch {
    /// Creates new switch with given `description`
    pub fn new(description: &str) -> Self {
        Self {
            state: SwitchState::Off,
            description: String::from(description),
            power_consumption: 0.0,
        }
    }

    /// Returns description of the switch
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Switches state of the switch according `state` arg
    pub fn turn(&mut self, state: SwitchState) {
        self.state = state;
    }

    /// Returns current power consumption of the switch
    pub fn power_consumption(&self) -> f64 {
        self.power_consumption
    }
}
