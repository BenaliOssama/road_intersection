// mod cars;
// mod roads;
// mod traffic_lights;
// mod intersection;

//use intersection::Intersection;

use sdl2::{event::Event, keyboard::Keycode,
init, pixels::Color, rect::Rect, rect::Point};
use std::time::Instant;

fn main() {

    let sdl_context = init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas to draw on the window
    //  send draw commands to a renderer
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut y_pos = 0f32; // initial vertical position
    let speed = 60.0;
    let mut last_time = Instant::now();


    'running: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now ; 

        // Update y position: move down by 5 pixels each frame
        y_pos += speed * dt ;
        if y_pos > 600.0 {
            y_pos = 0.0 ;
        }
        // Clear screen with black
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();


        // Set draw color to white for the line
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw line
        canvas.draw_line(Point::new(375, 0), Point::new(375, 600)).unwrap();
        canvas.draw_line(Point::new(425, 0), Point::new(425, 600)).unwrap();


        // Set draw color to yellow
        canvas.set_draw_color(Color::RGB(255, 255, 0));

        // Draw a 50x50 square at fixed x=375, moving y=y_pos
        let square = Rect::new(375, y_pos as i32, 50, 50);
        canvas.fill_rect(square).unwrap();


        // SDL2 uses double buffering for smooth graphics.
        // swaps the hidden buffer with the visible one
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            }
        }
    }
}

