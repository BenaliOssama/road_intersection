use crate::cars::Car;
use crate::{roads::Road, traffic_lights::TrafficLight};
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone)]
pub struct Line {
    pub road: Road,
    pub traffic_light: TrafficLight,
    pub direction: Direction,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Line {
    pub fn new(size: (i32, i32), direction: Direction) -> Self {
        let road = Road::new(size, direction.clone());
        let traffic_light = TrafficLight::new(direction.clone());
        Line {
            road,
            traffic_light,
            direction,
        }
    }

    pub fn draw(&self, direction: Direction, canvas: &mut Canvas<Window>) {
        self.road.draw(direction.clone(), canvas);
        self.traffic_light
            .draw((800, 600),  canvas);
    }

    pub fn update(&mut self, dt: f32, is_green: bool) {
        self.road.update(dt, is_green);
        self.traffic_light.update(dt, is_green);
    }
    pub fn add_new_car(&mut self) {
        self.road.add_new_car();
    }
    pub fn add_car(&mut self, car: Car) {
        self.road.add_car(car);
    }
    pub fn remove(&mut self, car: Car) {
        self.road.remove_car(car);
    }
    pub fn car_in_zone1(&self) -> Option<&Car> {
        return self.road.car_in_zone1();
    }
    pub fn car_in_zone2(&self) -> Option<&Car> {
        return self.road.car_in_zone2();
    }
    pub fn first_car_wait_time(&self) -> u64 {
        todo!()
    }
}
