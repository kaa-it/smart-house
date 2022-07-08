//! Module describes power switch device for smart house

use crate::command::Command;
use crate::response::Response;
use std::fmt::{self, Display};

/// Describes state of power switch
#[derive(Debug, Display, Clone)]
pub enum SwitchState {
    On,
    Off,
}

impl TryFrom<u8> for SwitchState {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SwitchState::Off),
            1 => Ok(SwitchState::On),
            _ => Err("Failed to convert value to SwitchState"),
        }
    }
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

    /// Creates new switch with full setting
    pub fn from_settings(description: &str, state: SwitchState, power_consumption: f64) -> Self {
        Self {
            state,
            description: String::from(description),
            power_consumption,
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

    /// Process commands for the switch
    pub fn process_command(&mut self, command: Command) -> Response {
        match command {
            Command::TurnOn => {
                self.state = SwitchState::On;
                Response::Ok
            }
            Command::TurnOff => {
                self.state = SwitchState::Off;
                Response::Ok
            }
            Command::IsEnabled => match self.state {
                SwitchState::On => Response::Enabled,
                SwitchState::Off => Response::Disabled,
            },
            Command::GetPower => match self.state {
                SwitchState::On => Response::Power(self.power_consumption),
                SwitchState::Off => Response::Power(0.0),
            },
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_power_switch() {
        const POWER_SWITCH_INFO: &str =
            r#"Power Switch (state: Off, description: "Bathroom", power consumption: 0)"#;

        let power_switch = PowerSwitch::new("Bathroom");

        let power_switch_info = format!("{}", power_switch);

        assert_eq!(power_switch_info, POWER_SWITCH_INFO);
    }
}
