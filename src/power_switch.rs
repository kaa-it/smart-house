#[derive(Debug)]
pub enum SwitchState {
    On,
    Off,
}

#[derive(Debug)]
pub struct PowerSwitch {
    state: SwitchState,
    description: String,
    power: f64,
}

impl PowerSwitch {
    pub fn new(description: &str) -> Self {
        Self {
            state: SwitchState::Off,
            description: String::from(description),
            power: 0.0,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn turn(&mut self, state: SwitchState) {
        self.state = state;
    }

    pub fn current_power(&self) -> f64 {
        self.power
    }
}
