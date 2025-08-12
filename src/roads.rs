use crate::cars::Car;


    pub struct Road {
        pub cars: Vec<Car>,
    }

    impl Road {
        pub fn new() -> Self {
            Road { cars: Vec::new() }
        }

        pub fn add_car(&mut self, car: Car) {
            self.cars.push(car);
        }

        pub fn update(&mut self, dt: f32) {
            for car in &mut self.cars {
                car.update(dt);
            }
        }

        pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            for car in &self.cars {
                car.draw(canvas);
            }
        }
    }

