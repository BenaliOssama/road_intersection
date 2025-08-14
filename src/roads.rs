use crate::cars::{Car, CarColor};
use crate::Direction;

use rand::Rng;

#[derive(Clone)]
pub struct Road {
    pub cars: Vec<Car>,
    pub safty: f32,
    pub stop_lign: f32,
    direction: Direction,
    pub size: (i32, i32),
    // zone 1
    // zone 2
}

impl Road {
    pub fn firt_car_wait_time(&self) -> u64 {
        return self.first_car_befor_line().wait_time();
    }
    pub fn first_car_befor_line(&self) -> Car {
        todo!()
    }
    pub fn update(&mut self, dt: f32, is_green: bool) {
        // remove cars that out of the visible road
        self.cars.retain(|car| {
            let in_bounds = match self.direction {
                //car.y + car.speed - 50.0 <= (self.size.1 as f32)
                Direction::North => car.y + car.speed - 50.0 <= self.size.1 as f32,
                Direction::South => car.y + car.speed > 0.0, // - car.speed + 50.0 >= 0.0,
                Direction::East => car.x + car.speed - 50.0 <= self.size.0 as f32,
                Direction::West => car.x + car.speed - 50.0 <= self.size.0 as f32, // 0.0, //- car.speed + 50.0 >= 0.0,
            };
            in_bounds
        });

        let dir = self.direction.clone();

        // Precompute the before_light flags to avoid borrowing `self` during mutable iteration
        let before_light_flags: Vec<bool> = self
            .cars
            .iter()
            .map(|car| self.is_before_light(car.clone()))
            .collect();
        let moves: Vec<bool> = self
            .cars
            .iter()
            .enumerate()
            .map(|(i, car)| self.car_can_move(car.clone(), is_green, dt, i))
            .collect();

        for ((car, before_light), can_move) in
            self.cars.iter_mut().zip(before_light_flags).zip(moves)
        {
            if can_move || !before_light {
                car.start();
                car.update(dt, dir.clone());
            } else {
                car.stop();
            }
        }
    }
    //
    pub fn car_can_move(&self, car: Car, is_green: bool, dt: f32, index: usize) -> bool {
        // is there a safe distance
        // is the red and in line
        if !self.is_safe_to_move(car, is_green, dt, index) {
            println!("car can nooooot move");
            return false;
        }
        println!("car can mofe");
        true
    }

    pub fn is_safe_to_move(&self, car: Car, is_green: bool, dt: f32, index: usize) -> bool {
        if self.car_in_line(car, dt) && !is_green {
            print!("because car is in line");
            return false;
        }

        // is there safe distance
        match self.direction {
            Direction::North => {
                if index > 0 {
                    if car.y + self.safty + dt >= self.cars[index - 1].y {
                        return false;
                    }
                }
            }
            Direction::South => {
                if index > 0 {
                    if car.y - self.safty - dt <= self.cars[index - 1].y {
                        return false;
                    }
                }
            }

            Direction::West => {
                if index > 0 {
                    if car.x + self.safty + dt >= self.cars[index - 1].x {
                        return false;
                    }
                }
            }
            Direction::East => {
                if index > 0 {
                    if car.x - self.safty - dt <= self.cars[index - 1].x {
                        return false;
                    }
                }
            }
        }
        true
    }
    // is the car in stop line
    pub fn car_in_line(&self, car: Car, dt: f32) -> bool {
        return self.car_in_zone3(car);
    }
}

impl Road {
    pub fn new(size: (i32, i32), direction: Direction) -> Self {
        let (w, h) = size;
        let center = ((w as f32) / 2.0, (h as f32) / 2.0);
        let stop_offset = 50.0; // distance from intersection center

        let stop_lign = match direction {
            Direction::North => center.1 - stop_offset,
            Direction::South => center.1 + stop_offset,

            Direction::West => center.0 - stop_offset,
            Direction::East => center.0 + stop_offset,
        };
        println!("road {:?} and it's light in {:?}", direction, stop_lign);
        Road {
            cars: Vec::new(),
            safty: 50.0 + 50.0,
            stop_lign,
            direction,
            size,
        }
    }

    pub fn car_in_zone1(&self) -> Option<&Car> {
        let zone_length = 50.0;
        let stop = self.stop_lign;

        self.cars.iter().find(|car| match self.direction {
            Direction::North => {
                let car_head = car.y;

                car_head >= stop && car_head <= stop + zone_length
            }
            Direction::South => {
                let car_head = car.y;
                let stop = stop - 50.0;
                car_head <= stop && car_head >= stop - zone_length
            }

            Direction::East => {
                let car_head = car.x;

                car_head <= stop && car_head >= stop - zone_length
            }

            _ => false,
            // Direction::West => {
            //     let car_head = car.x;

            //     car_head >= stop && car_head <= stop + zone_length
            // }
        })
    }
    pub fn car_in_zone2(&self) -> Option<&Car> {
        let zone_length = 50.0;
        let stop = self.stop_lign;

        self.cars.iter().find(|car| match self.direction {
            Direction::North => {
                let car_head = car.y;
                let stop = stop + 50.0;
                car_head >= stop && car_head <= stop + zone_length && car.turned == false
            }
            Direction::South => {
                let car_head = car.y;
                let stop = stop - 100.0;
                car_head <= stop && car_head >= stop - zone_length && car.turned == false
            }

            Direction::East => {
                let car_head = car.x;
                let stop = stop - 50.0;
                car_head <= stop && car_head >= stop - zone_length && car.turned == false
            }

            Direction::West => {
                let car_head = car.x;

                car_head >= stop && car_head <= stop + zone_length && car.turned == false
            }
            _ => false,
        })
    }

    pub fn car_in_zone3(&self, car: Car) -> bool {
        let zone_length = 50.0;
        let stop = self.stop_lign;

        match self.direction {
            Direction::North => {
                let car_head = car.y + 50.0;

                car_head < stop && car_head > stop - 2.0
            }
            Direction::South => {
                let car_head = car.y;
                car_head > stop && car_head < stop + 2.0
            }
            Direction::East => car.x > stop && car.x < stop + 2.0,

            Direction::West => {
            
                car.x + 50.0 < stop && car.x + 50.0 > stop - 2.0
            }
        }
    }
}

impl Road {
    pub fn add_new_car(&mut self) {
        let vehicle_length = 50.0;
        let safety_gap = 30.0;
        let s_min = vehicle_length + safety_gap;
        let lane_length = 800.0;
        let capacity = (lane_length / s_min) as usize;
        let num = rand::thread_rng().gen_range(0..3);
        // this need safty check to add a car
        let (w, h) = self.size;
        let center = ((w as f32) / 2.0, (h as f32) / 2.0);
        // use size and direction

        let color = match num {
            0 => CarColor::Blue,
            1 => CarColor::White,
            2 => CarColor::Yellow,
            _ => panic!("never more than what should be: {}", num),
        };
        let (x, y) = match self.direction {
            Direction::North => (center.0 as i32 - 50, 50),
            Direction::South => (center.0 as i32, self.size.1 - 50),
            Direction::East => (w - 50, center.1 as i32 - 50),
            Direction::West => (1, center.1 as i32),
        };
        let car = Car::new(color, x as f32, y as f32, 60.0);

        if self.cars.len() == 0 {
            self.cars.push(car);
            return;
        }

        let last_car = self.cars.last().unwrap();

        let distance_ok = match self.direction {
            Direction::North => (last_car.y - y as f32).abs() >= s_min,
            Direction::South => (y as f32 - last_car.y).abs() >= s_min,
            Direction::East => (x as f32 - last_car.x).abs() >= s_min,
            Direction::West => (last_car.x - x as f32).abs() >= s_min,
        };

        if distance_ok && self.cars.len() < capacity {
            self.cars.push(car);
        }
    }

    pub fn last_car_before_light(&self) -> usize {
        if self.cars.len() == 0 {
            return 0;
        }
        match self.direction {
            Direction::North => {
                for (i, car) in self.cars.iter().enumerate() {
                    if car.y < self.stop_lign {
                        return i;
                    }
                }
                0
            }
            Direction::South => {
                for (i, car) in self.cars.iter().enumerate() {
                    if car.y > self.stop_lign {
                        return i;
                    }
                }
                0
            }
            Direction::West => {
                for (i, car) in self.cars.iter().enumerate() {
                    if car.x < self.stop_lign {
                        return i;
                    }
                }
                0
            }
            Direction::East => {
                for (i, car) in self.cars.iter().enumerate() {
                    if car.x > self.stop_lign {
                        return i;
                    }
                }
                0
            }
        }
    }

    pub fn add_car(&mut self, car: Car) {
        println!("add car to : {:?}", self.direction);
        let index = if self.last_car_before_light() == 0 {
            0
        } else {
            self.last_car_before_light() - 1
        };

        self.cars.insert(index, car);
    }

    pub fn is_before_light(&self, car: Car) -> bool {
        // is the car befor trafic light return true
        match self.direction {
            Direction::North => car.y + 50.0 < self.stop_lign,
            Direction::South => car.y > self.stop_lign,

            Direction::West => car.x < self.stop_lign,
            Direction::East => car.x > self.stop_lign,
        }
    }

    pub fn remove_car(&mut self, car: Car) {
        // remove the car from road

        self.cars.retain(|c| *c != car);
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
