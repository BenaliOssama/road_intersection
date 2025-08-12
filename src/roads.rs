use crate::cars::Car;
    
    
pub struct Road {
    pub cars: Vec<Car>,
    safty: f32,
}

impl Road {
    pub fn new() -> Self {
        Road { cars: Vec::new(), safty: 50.0 + 10.0 }
    }

    pub fn add_car(&mut self, car: Car) {
        if self.cars.len() == 0 {
            self.cars.push(car);
            return;
        }

        let last_car_pos : f32 = self.cars[self.cars.len() -1].y;
        println!("last car y :{}",last_car_pos );
        if last_car_pos >= self.safty {
            self.cars.push(car);
        }
    }

    // fn remove_car(&mut self, car: Car) {
    //     if self.last_car.is_none(){
    //         self.cars.push(car);
    //     }else if self.last_car.as_ref().unwrap().y >= self.safty {
    //         self.cars.push(car);
    //     }
    // }

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

