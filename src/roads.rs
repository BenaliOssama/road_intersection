use crate::cars::{self, Car};
use crate::Direction;
use sdl2::pixels::Color;
use sdl2::rect::Point;

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
        todo!()
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
            car.update(dt);
        }
    }

    pub fn draw(
        &self,
        direction: Direction,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let common: ((i32, i32), (i32, i32)) =
            if direction == Direction::North || direction == Direction::South {
                (
                   (self.size.0 / 2, 0),
                    (self.size.0 / 2, self.size.1),
                )
            } else {
                (
                   (0, self.size.1 / 2),
                   (self.size.0, self.size.1 / 2),
                )
            };

        canvas.draw_line(common.0, common.1).unwrap();

        let other = match direction {
            Direction::North => {
                (Point::new(common.0.0 - 50, common.0.1),Point::new(common.1.0 - 50, common.1.1))
            }
            Direction::North => {
                (Point::new(0, 0), Point::new(0,0))
            }
                        Direction::North => {
                (Point::new(0, 0), Point::new(0,0))
            }
                        Direction::North => {
                (Point::new(0, 0), Point::new(0,0))
            }
            _ => panic!("should be a valid point"),
        };

        canvas
            .draw_line(other.0, other.1)
            .unwrap();
        // switch case

        for car in &self.cars {
            car.draw(canvas);
        }
    }
}
