mod cars;
mod intersection;
mod roads;
mod traffic_lights;
mod lines;

use rand::Rng;
use crate::intersection::*;

use crate::lines::*;//Line::Direction;

use sdl2::{event::Event, init, keyboard::Keycode, pixels::Color};
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

    let mut intersection = Intersection::new((800, 600));
    let mut last_time = Instant::now();

    'running: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        intersection.update(dt);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();



        intersection.draw(&mut canvas);
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    intersection.add_car_from_direction(Direction::South);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    intersection.add_car_from_direction(Direction::North);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    intersection.add_car_from_direction(Direction::East);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    intersection.add_car_from_direction(Direction::West);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                     let num = rand::thread_rng().gen_range(0..4);
                     match num {
                        0 =>  intersection.add_car_from_direction(Direction::West),
                        1 =>  intersection.add_car_from_direction(Direction::East), 
                        2 =>  intersection.add_car_from_direction(Direction::North), 
                        3 =>  intersection.add_car_from_direction(Direction::South),
                        _ => (),
                        
                     }
                    intersection.add_car_from_random_direction();
                }
                _ => {}
            }
        }
    }
}
