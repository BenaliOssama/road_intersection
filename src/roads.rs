use crate::cars::Car;

pub struct Lane {
    pub cars: Vec<Car>,
    pub length: f32,
    pub capacity: usize,
}

impl Lane {
    pub fn new(length: f32, capacity: usize) -> Self {
        Self {
            cars: Vec::new(),
            length,
            capacity,
        }
    }

    pub fn can_spawn_car(&self) -> bool {
        self.cars.len() < self.capacity
    }

    pub fn add_car(&mut self, car: Car) {
        if self.can_spawn_car() {
            self.cars.push(car);
        }
    }
}

pub struct Road {
    pub northbound: Lane,
    pub southbound: Lane,
    pub eastbound: Lane,
    pub westbound: Lane,
}

impl Road {
    pub fn new(lane_length: f32, lane_capacity: usize) -> Self {
        Self {
            northbound: Lane::new(lane_length, lane_capacity),
            southbound: Lane::new(lane_length, lane_capacity),
            eastbound: Lane::new(lane_length, lane_capacity),
            westbound: Lane::new(lane_length, lane_capacity),
        }
    }
}

