

use crate::cars::{Car, CarColor};
use crate::lines::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;


// Direction enum with conversion to string

impl Direction {
    pub fn to_str(&self) -> &'static str {
        match self {
            Direction::North => "T_R",
            Direction::South => "B_R",
            Direction::East => "T_L",
            Direction::West => "B_L",
        }
    }
}


// Intersection struct to manage lines and add cars
pub struct Intersection {
    elapsed: f32,
    lines: HashMap<Direction, Line>,
}

impl Intersection {
    pub fn new(size: (i32, i32)) -> Self {
        let mut lines = HashMap::new();
        // Directly create and insert lines without cloning
        lines.insert(Direction::North, Line::new(size, Direction::North));
        lines.insert(Direction::South, Line::new(size, Direction::South));
        lines.insert(Direction::West, Line::new(size, Direction::West));
        lines.insert(Direction::East, Line::new(size, Direction::East));

        Intersection {
            lines,
            elapsed: 0.0,
        }
    }

    // Add car to a specific direction line
    pub fn add_car_from_direction(&mut self, dir: Direction) {
        // Convert direction to string to access the corresponding line
        if let Some(line) = self.lines.get_mut(&dir) {
            line.add_car(Car::new(CarColor::Yellow, 375, 0.0, 60.0));
        }
    }

    // Add car to a random direction line
    pub fn add_car_from_random_direction(&mut self) {
        // let mut rng = rand::rngs::ThreadRng::default();
        // if let Some((_key, line)) = self.lines[&mut rng] {
        //     line.add_car(Car::new(CarColor::Yellow, 375, 0.0, 60.0));
        // }
    }

    // Draw all lines in the intersection
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for (_, line) in &self.lines {
            line.draw(canvas);
        }
    }

    // Update all lines in the intersection
    pub fn update(&mut self, dt: f32) {
        // calculate then update
        let c = self.clock(dt);
        for (_, line) in &mut self.lines {
            line.update(dt, c);
        }
    }
    fn clock(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        if self.elapsed >= 3.0 {
            self.elapsed = 0.0;
            return true;
        }
        false
    }
}
