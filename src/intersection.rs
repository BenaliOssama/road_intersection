use crate::{roads::Road , traffic_lights::TrafficLight};
use crate::{cars::Car};//, traffic_lights::TrafficLight};

//use crate::cars;
use crate::cars::CarColor;

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Intersection {
    pub roads: Vec<Road>,
    pub traffic_lights: Vec<TrafficLight>,
}

impl Intersection {
    pub fn new(size: (i32, i32)) -> Self {
        let road = Road::new(size);
        let roads = vec![road.clone(), road];
        let traffic_lights = vec![TrafficLight::new((10, 10)); 4];

        Intersection { roads, traffic_lights }
    }

    pub fn add_car_from_direction(&mut self, _dir: Direction) {
        for road in &mut self.roads {
            road.add_car(Car::new(CarColor::Yellow, 375, 0.0, 60.0));
        }
    }
    
    pub fn add_car_from_random_direction(&mut self) {
        // Randomly pick a direction and add car
    }
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for road in &self.roads {
            road.draw(canvas);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for light in &mut self.traffic_lights {
            light.update(dt);
        }
        for road in &mut self.roads {
            road.update(dt);
        }
    }
}

