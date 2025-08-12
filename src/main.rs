mod roads;
mod cars;
mod intersection;

use sdl2::{event::Event, keyboard::Keycode, init, pixels::Color, rect::Point};
use std::time::Instant;

use crate::cars::{Car, CarColor};
use crate::roads::Road;

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

    // Create a road and add some cars
    let mut road = Road::new((800, 600));

    let mut last_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        road.update(dt);

        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw lane lines
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(Point::new(375, 0), Point::new(375, 600)).unwrap();
        canvas.draw_line(Point::new(425, 0), Point::new(425, 600)).unwrap();

        // Draw all cars on the road
        road.draw(&mut canvas);

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                // quite
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                // add car up
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    road.add_car(Car::new(CarColor::Yellow, 375, 0.0, 60.0));
                }
                _ => {}
            }
        }
    }
}
