use sdl2::{event::Event, keyboard::Keycode, init, pixels::Color, rect::Point};
use std::time::Instant;

fn main() {
    let sdl_context = init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut car = car::Car::new(car::CarColor::Yellow, 375, 0.0, 60.0);

    let mut last_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        // Update car
        car.update(dt);

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw lane lines
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(Point::new(375, 0), Point::new(375, 600)).unwrap();
        canvas.draw_line(Point::new(425, 0), Point::new(425, 600)).unwrap();

        // Draw car
        car.draw(&mut canvas);

        // Show frame
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
    }
}

mod car {
    use sdl2::{pixels::Color, rect::Rect};

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    pub enum CarColor {
        Yellow,
        Blue,
        Red,
        White,
    }

    pub struct Car {
        pub color: CarColor,
        pub x: i32,
        pub y: f32,     // f32 for smooth movement
        pub speed: f32, // f32 for dt multiplication
        pub moving: bool,
    }

    impl Car {
        pub fn new(color: CarColor, x: i32, y: f32, speed: f32) -> Self {
            Car { color, x , y , speed , moving: true}
        }

        pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
            let color = match self.color {
                CarColor::Yellow => Color::RGB(255, 255, 0),
                CarColor::Blue => Color::RGB(0, 0, 255),
                CarColor::Red => Color::RGB(255, 0, 0),
                CarColor::White => Color::RGB(255, 255, 255),
            };
            canvas.set_draw_color(color);
            let rect = Rect::new(self.x, self.y as i32, 50, 50);
            canvas.fill_rect(rect).unwrap();
        }

        pub fn update(&mut self, dt: f32) {
            if !self.moving {
                return;
            }
            self.y += self.speed * dt;
            if self.y > 600.0 {
                self.y = 0.0;
            }
        }

        pub fn stop(&mut self){
            self.moving = false;
        }

        pub fn start(&mut self){
            self.moving = true;
        }
    }
}

