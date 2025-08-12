#[derive(Debug, Clone, Copy)]
pub enum CarColor {
    Yellow,
    Blue,
    Red,
    White,
}

pub struct Car {
    pub color: CarColor,
    pub velocity: f32,
    pub route: Route,
}

#[derive(Debug, Clone, Copy)]
pub enum Route {
    Left,
    Straight,
    Right,
}

impl Car {
    pub fn new(color: CarColor, velocity: f32, route: Route) -> Self {
        Car { color, velocity, route }
    }
}

