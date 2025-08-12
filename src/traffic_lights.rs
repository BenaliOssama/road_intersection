#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LightState {
    Red,
    Green,
}

#[derive(Clone)]
pub struct TrafficLight {
    pub state: LightState,
    pub timer: f32, // simple timer in ticks or ms
}

impl TrafficLight {
    pub fn new() -> Self {
        TrafficLight {
            state: LightState::Red,
            timer: 0.0,
        }
    }

    pub fn switch(&mut self) {
        self.state = match self.state {
            LightState::Red => LightState::Green,
            LightState::Green => LightState::Red,
        };
        self.timer = 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.timer += delta_time;
        // Implement logic to switch state based on timer here
    }
}

