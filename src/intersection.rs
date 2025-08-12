use crate::{roads::Road, traffic_lights::TrafficLight};
use crate::{cars::Car};
use crate::cars::CarColor;

pub enum Direction {
    North,
    South,
    East,
    West,
}

// Line module
pub mod lines {
    use crate::{roads::Road, traffic_lights::TrafficLight};
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    #[derive(Clone)]
    pub struct Line {
        pub road: Road,
        pub traffic_light: TrafficLight,
    }

    impl Line {
        pub fn new(size: (i32, i32)) -> Self {
            let road = Road::new(size);
            let traffic_light = TrafficLight::new((10, 10));
            Line { road, traffic_light }
        }

        pub fn draw(&self, canvas: &mut Canvas<Window>) {
            self.road.draw(canvas);
            self.traffic_light.draw(canvas);
        }

        pub fn update(&mut self, dt: f32) {
            self.traffic_light.update(dt);
            self.road.update(dt);
        }

        pub fn add_car(&mut self, car: crate::cars::Car) {
            self.road.add_car(car);
        }
    }
}

use lines::Line;

// Intersection struct
pub struct Intersection {
    pub lines: Vec<Line>,
}

impl Intersection {
    pub fn new(size: (i32, i32)) -> Self {
        let line = Line::new(size);
        let lines = vec![line.clone(), line.clone(), line.clone(), line];
        Intersection { lines }
    }

    pub fn add_car_from_direction(&mut self, _dir: Direction) {
        // Example: always add to first line
        if let Some(line) = self.lines.get_mut(0) {
            line.add_car(Car::new(CarColor::Yellow, 375, 0.0, 60.0));
        }
    }
    
    pub fn add_car_from_random_direction(&mut self) {
        // use rand::seq::SliceRandom;
        // let mut rng = rand::thread_rng();
        // if let Some(line) = self.lines.choose(&mut rng) {
        //     line.add_car(Car::new(CarColor::Yellow, 375, 0.0, 60.0));
        // }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for line in &self.lines {
            line.draw(canvas);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for line in &mut self.lines {
            line.update(dt);
        }
    }
}

