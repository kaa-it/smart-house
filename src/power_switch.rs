//! Module describes power switch device for smart house

use std::fmt::{self, Display};

/// Describes state of power switch
#[derive(Debug, Display, Clone)]
pub enum SwitchState {
    On,
    Off,
}

/// Describes smart power switch
#[derive(Debug, Clone)]
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

impl Display for PowerSwitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Power Switch (state: {}, description: \"{}\", power consumption: {})",
            self.state, self.description, self.power_consumption
        )
    }
}
