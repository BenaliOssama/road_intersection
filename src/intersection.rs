use crate::cars::{Car, CarColor};
use crate::lines::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;

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
            line.add_new_car();
        }
    }

    // Draw all lines in the intersection
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for (direction, line) in &self.lines {
            line.draw(direction.clone(), canvas);
        }
    }
    pub fn update(&mut self, dt: f32) {
        let c = self.clock(dt);

        // First, figure out which cars to move
        let mut moves = Vec::new();
        for (direct, line) in &mut self.lines {
            if let Some(car) = line.car_in_zone() {
                println!("week week a 3ibad lah rani f zone");
                let take_line = what_line_to_take(&car.color, direct);
                moves.push((direct.clone(), take_line, car));
                line.remove(car.clone());
            }
            line.update(dt, c);
        }

        // Now perform the moves
        for (_, take_line, car) in moves {
            if let Some(line) = self.lines.get_mut(&take_line) {
                line.add_car(car);
            }
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

fn what_line_to_take(car_color: &CarColor, comming_from: &Direction) -> Direction {
    match car_color {
        CarColor::Yellow => {
            return comming_from.clone();
        }
        CarColor::White => {
            // turn right
            match comming_from {
                Direction::North => return Direction::East,
                Direction::East => return Direction::South,
                Direction::South => return Direction::West,
                Direction::West => return Direction::North,
            }
        }
        CarColor::Blue => match comming_from {
            Direction::North => return Direction::West,
            Direction::East => return Direction::South,
            Direction::South => return Direction::East,
            Direction::West => return Direction::North,
        },
    }
}
