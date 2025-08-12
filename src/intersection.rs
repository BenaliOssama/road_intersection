use crate::{roads::Road, traffic_lights::TrafficLight};

pub struct Intersection {
    pub roads: Vec<Road>,
    pub traffic_lights: Vec<TrafficLight>,
}

// impl Intersection {
//     pub fn new() -> Self {
//         // create 2 roads crossing each other (simplified)
//         let road = Road::new(100.0, 10);
//         let roads = vec![road.clone(), road];
//
//         // create traffic lights for lanes
//         let traffic_lights = vec![TrafficLight::new(); 8]; // example: 4 lanes x 2 lights
//
//         Intersection { roads, traffic_lights }
//     }
//
//     pub fn update(&mut self, delta_time: u32) {
//         for light in &mut self.traffic_lights {
//             light.update(delta_time);
//         }
//
//         // update roads and cars logic here
//     }
// }
//
