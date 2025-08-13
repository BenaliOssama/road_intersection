use crate::cars::{Car, CarColor};
use crate::Direction;

use rand::Rng;

#[derive(Clone)]
pub struct Road {
    pub cars: Vec<Car>,
    pub safty: f32,
    pub stop_lign: (f32, f32),
    direction: Direction,
    pub size: (i32, i32),
}

impl Road {
    pub fn new(size: (i32, i32), direction: Direction) -> Self {
        let (w, h) = size;
        let center = ((w as f32) / 2.0, (h as f32) / 2.0);
        let stop_offset = 50.0; // distance from intersection center

        let stop_lign = match direction {
            Direction::North => (center.0, center.1 - stop_offset),
            Direction::South => (center.0, center.1 + stop_offset),
            Direction::West => (center.0 - stop_offset, center.1),
            Direction::East => (center.0 + stop_offset, center.1),
        };

        Road {
            cars: Vec::new(),
            safty: 50.0 + 15.0,
            stop_lign,
            direction,
            size,
        }
    }

    pub fn add_new_car(&mut self) {
        let num = rand::thread_rng().gen_range(0..3);
        // this need safty check to add a car
        let (w, h) = self.size;
        let center = ((w as f32) / 2.0, (h as f32) / 2.0);
        // use size and direction
        let (x, y) = match self.direction {
            Direction::North => (center.0 as i32 - 50, 50),
            Direction::South => (center.0 as i32, self.size.1 - 50),
            Direction::West => (w, center.1 as i32 - 50),
            Direction::East => (0, center.1 as i32),
        };
        let color = match num {
            0 => CarColor::Blue,
            1 => CarColor::White,
            2 => CarColor::Yellow,
            _ => panic!("never more than what should be: {}", num),
        };
        let car = Car::new(color, x as f32, y as f32, 60.0);

        if self.cars.len() == 0 {
            self.cars.push(car);
            return;
        }

        let last_car_pos: f32 = self.cars[self.cars.len() - 1].y;
        if last_car_pos >= self.safty {
            self.cars.push(car);
        }
    }

    pub fn add_car(&mut self, car: Car) {
        // find indes of the first car befor traffic light and insert at that index;
        if self.cars.len() == 0 {
            self.cars.push(car);
            return;
        }

        let last_car_pos: f32 = self.cars[self.cars.len() - 1].y;
        if last_car_pos >= self.safty {
            self.cars.push(car);
        }
    }

    pub fn is_before_light(car: Car) -> bool {
        // is the car befor trafic light return true
        todo!()
    }

    pub fn remove_car(&mut self, car: Car) {
        // remove the car from road

        self.cars.retain(|c| *c != car);
    }

    pub fn update(&mut self, dt: f32) {
        self.cars.retain(|car| {
            let in_bounds = car.y + car.speed - 50.0 <= (self.size.1 as f32);
            if in_bounds {
                // update the car only if it's still in bounds
                // (assuming car.update() mutates car)
                // Since retain borrows immutably, update separately below
                true
            } else {
                false
            }
        });

        // Now update all cars still in the vector
        for car in &mut self.cars {
            car.update(dt, self.direction.to_owned());
        }
    }

    pub fn draw(
        &self,
        direction: Direction,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        use sdl2::pixels::Color;
        use sdl2::rect::Point;

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Main center line
        let common: ((i32, i32), (i32, i32)) =
            if direction == Direction::North || direction == Direction::South {
                ((self.size.0 / 2, 0), (self.size.0 / 2, self.size.1))
            } else {
                ((0, self.size.1 / 2), (self.size.0, self.size.1 / 2))
            };

        canvas.draw_line(common.0, common.1).unwrap();

        // Second line to represent the lane width
        let lane_offset = 50; // half of lane width
        let other = match direction {
            Direction::North => (
                Point::new(common.0 .0 - lane_offset, common.0 .1),
                Point::new(common.1 .0 - lane_offset, common.1 .1),
            ),
            Direction::South => (
                Point::new(common.0 .0 + lane_offset, common.0 .1),
                Point::new(common.1 .0 + lane_offset, common.1 .1),
            ),
            Direction::East => (
                Point::new(common.0 .0, common.0 .1 - lane_offset),
                Point::new(common.1 .0, common.1 .1 - lane_offset),
            ),
            Direction::West => (
                Point::new(common.0 .0, common.0 .1 + lane_offset),
                Point::new(common.1 .0, common.1 .1 + lane_offset),
            ),
        };

        canvas.draw_line(other.0, other.1).unwrap();

        // Draw all cars on this road
        for car in &self.cars {
            car.draw(canvas);
        }
    }
}
