use crate::cars::CarColor;
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
        println!("car comming from {:?}", dir);
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
}

impl Intersection {
    pub fn update(&mut self, dt: f32) {
        let is_green = self.clock(dt);
        println!("is green: {:?}", is_green);
        // First, figure out which cars to move
        let mut moves = Vec::new();
        for (direct, line) in &mut self.lines {
            if let Some(car) = line.car_in_zone1() {
                if car.color == CarColor::White {
                    let take_line = what_line_to_take(&car.color, direct);
                    println!("take line: {:?}", take_line);
                    moves.push((take_line, {
                        let mut  c =car.clone();
                        c.color = CarColor::Yellow;
                        c
                    }));
                    line.remove(car.clone());
                }
            }
            if let Some(car) = line.car_in_zone2() {
                // todo!()
                if car.color == CarColor::Blue {
                    let take_line = what_line_to_take(&car.color, direct);
                    println!("take line: {:?}", take_line);
                    moves.push((take_line, {
                        let mut  c =car.clone();
                        c.color = CarColor::Yellow;
                        c
                    }));
                    line.remove(car.clone());
                }
            }
            if is_green == *direct {
                line.update(dt, true);
                // line.update(dt, false);
            } else {
                line.update(dt, false);
            }
        }

        // Now perform the moves
        for (take_line, car) in moves {
            if let Some(line) = self.lines.get_mut(&take_line) {
                line.add_car(car);
            }
        }
    }

    fn clock(&self, dt: f32) -> Direction {
        self.lines
            .iter()
            .max_by(|(_, line), (_, line1)| {
                // Replace with urgency calculation
                let waiting_time = line.first_car_wait_time();
                // println!("waiting time: {:?}", waiting_time);
                let car_count = line.road.cars.len();
                if (waiting_time as u64 + car_count as u64 * 10) > (line1.first_car_wait_time() as u64 + line1.road.cars.len() as u64 * 10) {
                    return std::cmp::Ordering::Greater;
                } else if  (waiting_time as u64 + car_count as u64 * 10) < (line1.first_car_wait_time() as u64 + line1.road.cars.len() as u64 * 10){
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Equal;
                }
            })
            .map(|(direction, _)| direction.clone())
            .expect("how the fuck did this happen?")
        // Direction::West
    }
}

fn what_line_to_take(car_color: &CarColor, comming_from: &Direction) -> Direction {
    println!("car comming from : {:?}", comming_from);
    match car_color {
        CarColor::Yellow => {
            return comming_from.clone();
        }
        CarColor::White => match comming_from {
            // turn left
            Direction::North => return Direction::East,
            Direction::East => return Direction::South,
            Direction::South => return Direction::West,
            Direction::West => return Direction::North,
        }
        CarColor::Blue => match comming_from {
            Direction::North => return Direction::West,
            Direction::West=> return Direction::South,
            Direction::South=> return Direction::East,
            Direction::East=> return Direction::North,
        },
    }
}
