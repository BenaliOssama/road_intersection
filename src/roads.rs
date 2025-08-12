use crate::cars::Car;
    
    
pub struct Road {
    pub cars: Vec<Car>,
    safty: f32,
    size: (i32, i32),
}

impl Road {
    pub fn new(size: (i32, i32)) -> Self {
        Road { cars: Vec::new(), safty: 50.0 + 15.0, size }
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
        self.cars.retain(|car| {
            let in_bounds = car.y + car.speed - 50.0 <= self.size.1 as f32;
            if in_bounds {
                // update the car only if it's still in bounds
                // (assuming car.update() mutates car)
                // Since retain borrows immutably, update separately below
                true
            } else {
                false
            }
        });

        // Now update all cars still in the vector
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

