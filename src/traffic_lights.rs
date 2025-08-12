use sdl2::{pixels::Color, rect::Rect};

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
    pos : (i32, i32),
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
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = Color::RGB(255, 255, 0);

        canvas.set_draw_color(color);

        let rect = Rect::new(self.pos.0 , self.pos.1, 50, 50);

        canvas.fill_rect(rect).unwrap();
    }

    #[allow(dead_code)]
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

