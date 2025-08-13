use crate::{roads::Road};
use sdl2::video::Window;
use sdl2::render::Canvas;
use std::collections::HashMap;
use crate::cars::{Car, CarColor};

// Direction enum with conversion to string
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn to_str(&self) -> &'static str {
        match self {
            Direction::North => "T_R",
            Direction::South => "B_R",
            Direction::East  => "T_L",
            Direction::West  => "B_L",
        }
    }
}

// Line module for handling roads and traffic lights
pub mod lines {
    use crate::{roads::Road,traffic_lights::TrafficLight};
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
            self.traffic_light.draw((325, 250), canvas);
        }

        pub fn update(&mut self, dt: f32, is_open: bool) {
            self.traffic_light.update(dt, is_open);
            self.road.update(dt);
        }

        pub fn add_car(&mut self, car: crate::cars::Car) {
            self.road.add_car(car);
        }
    }
}

use lines::Line;

// Intersection struct to manage lines and add cars
pub struct Intersection {
    elapsed: f32,
    lines: HashMap<&'static str, Line>,
}

impl Intersection {
    pub fn new(size: (i32, i32)) -> Self {
        let mut lines = HashMap::new();
        // Directly create and insert lines without cloning
        lines.insert("T_R", Line::new(size));
        lines.insert("T_L", Line::new(size));
        lines.insert("B_R", Line::new(size));
        lines.insert("B_L", Line::new(size));

        Intersection { lines, elapsed: 0.0 }
    }

    // Add car to a specific direction line
    pub fn add_car_from_direction(&mut self, dir: Direction) {
        // Convert direction to string to access the corresponding line
        if let Some(line) = self.lines.get_mut(dir.to_str()) {
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
        for (_, line)  in &mut  self.lines {
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
