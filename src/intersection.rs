use crate::{roads::Road , traffic_lights::TrafficLight};
use crate::{cars::Car};//, traffic_lights::TrafficLight};

pub struct Intersection {
    pub roads: Vec<Road>,
    pub traffic_lights: Vec<TrafficLight>,
}

impl Intersection {
    pub fn new(size: (i32, i32)) -> Self {
        // create 2 roads crossing each other (simplified)
        let road = Road::new(size);
        let roads = vec![road.clone(), road];

        //create traffic lights for lanes
        let traffic_lights = vec![TrafficLight::new(); 4]; // example: 4 lanes x 2 lights

        Intersection { roads, traffic_lights }
    }
    
    pub fn update(&mut self, dt: f32) {
        for light in &mut self.traffic_lights {
            light.update(dt);
        }
        for mut road in &mut self.roads {
            road.update(dt);
        } 
    }
}

