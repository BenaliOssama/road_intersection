use sdl2::{ pixels::Color, rect::Rect };
use crate::Direction;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LightState {
    Red,
    Green,
}

#[derive(Clone)]
pub struct TrafficLight {
    pub state: LightState,
    pub timer: f32, // simple timer in ticks or ms
    pos: (i32, i32),
}

impl TrafficLight {
    pub fn new(pos: (i32, i32)) -> Self {
        TrafficLight {
            state: LightState::Red,
            timer: 0.0,
            pos: pos,
        }
    }
    #[allow(dead_code)]
    pub fn draw(
        &self,
        size: (i32, i32),
        direction: Direction,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
    ) {
        let (w, h) = size;
        let center = (w / 2, h / 2);
        // let offset = 60; // distance from center / stop line
        println!("{:?}", center);

        // Determine light position based on direction
        let pos = match direction {
            Direction::North => (center.0 - 100, center.1 - 100),// above intersection
            Direction::South => (center.0 + 50, center.1 + 50), // below intersection
            Direction::West => (center.0 + 50, center.1 - 100), // left of intersection
            Direction::East => (center.0 - 100, center.1 + 50), // right of intersection
            _ => (0, 0),
        };

        // Set color based on light state
        match self.state {
            LightState::Red => canvas.set_draw_color(Color::RED),
            LightState::Green => canvas.set_draw_color(Color::GREEN),
        }

        let rect = Rect::new(pos.0, pos.1, 50, 50);
        canvas.draw_rect(rect).unwrap();
    }

    #[allow(dead_code)]
    pub fn switch(&mut self) {
        self.state = match self.state {
            LightState::Red => LightState::Green,
            LightState::Green => LightState::Red,
        };
        self.timer = 0.0;
    }

    pub fn update(&mut self, delta_time: f32, is_on: bool) {
        self.timer += delta_time;
        if is_on {
            self.switch();
        }
        // Implement logic to switch state based on timer here

    }
}
