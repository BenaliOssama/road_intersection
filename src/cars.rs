use sdl2::{pixels::Color, rect::Rect};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum CarColor {
    Yellow,
    Blue,
    Red,
    White,
}

pub struct Car {
    pub color: CarColor,
    pub x: i32,
    pub y: f32,     // f32 for smooth movement
    pub speed: f32, // f32 for dt multiplication
    pub moving: bool,
}

impl Car {
    pub fn new(color: CarColor, x: i32, y: f32, speed: f32) -> Self {
        Car { color, x , y , speed , moving: true}
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = match self.color {
            CarColor::Yellow => Color::RGB(255, 255, 0),
            CarColor::Blue => Color::RGB(0, 0, 255),
            CarColor::Red => Color::RGB(255, 0, 0),
            CarColor::White => Color::RGB(255, 255, 255),
        };
        canvas.set_draw_color(color);
        let rect = Rect::new(self.x, self.y as i32, 50, 50);
        canvas.fill_rect(rect).unwrap();
    }

    pub fn update(&mut self, dt: f32) {
        if !self.moving {
            return;
        }
        self.y += self.speed * dt;
        if self.y > 600.0 {
            self.y = 0.0;
        }
    }

    fn stop(&mut self){
        self.moving = false;
    }

    fn start(&mut self){
        self.moving = true;
    }
}

