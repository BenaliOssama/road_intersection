

    use crate::{roads::Road, traffic_lights::TrafficLight};
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    #[derive(Clone)]
    pub struct Line {
        pub road: Road,
        pub traffic_light: TrafficLight,
        pub direction: Direction,
    }

    #[derive(Eq, Hash, PartialEq, Clone)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }
    impl Line {
        pub fn new(size: (i32, i32), direction: Direction) -> Self {
            let road = Road::new(size);
            let traffic_light = TrafficLight::new((10, 10));
            Line {
                road,
                traffic_light,
                direction,
            }
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


