use sdl2::{pixels::Color, rect::Rect};

use crate::lines::Direction;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CarColor {
    Yellow,
    Blue,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Car {
    pub color: CarColor,
    pub x: f32,
    pub y: f32,     // f32 for smooth movement
    pub speed: f32, // f32 for dt multiplication
    pub moving: bool,
}

impl Car {
    pub fn wait_time(&self) -> u64{
        todo!()
    }
    pub fn new(color: CarColor, x: f32, y: f32, speed: f32) -> Self {
        Car {
            color,
            x,
            y,
            speed,
            moving: true,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = match self.color {
            CarColor::Yellow => Color::RGB(255, 255, 0),
            CarColor::Blue => Color::RGB(0, 0, 255),
            CarColor::White => Color::RGB(255, 255, 255),
        };
        canvas.set_draw_color(color);
        let rect = Rect::new(self.x as i32, self.y as i32, 50, 50);
        canvas.fill_rect(rect).unwrap();
    }

    pub fn update(&mut self, dt: f32, direction: Direction) {
        if !self.moving {
            return;
        }

        match direction {
            Direction::South => {
                self.y -= self.speed * dt; // move up
                if self.y < 0.0 {
                    self.y = 600.0; // wrap around (screen height)
                }
            }
            Direction::North => {
                self.y += self.speed * dt; // move down
                if self.y > 600.0 {
                    self.y = 0.0; // wrap around
                }
            }
            Direction::West => {
                self.x -= self.speed * dt; // move left
                if self.x < 0.0 {
                    self.x = 800.0; // wrap around (screen width)
                }
            }
            Direction::East => {
                self.x += self.speed * dt; // move right
                if self.x > 800.0 {
                    self.x = 0.0; // wrap around
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.moving = false;
    }

    #[allow(dead_code)]
    pub fn start(&mut self) {
        self.moving = true;
    }
}
