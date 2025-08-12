use sdl2::{event::Event, keyboard::Keycode, init, rect::Point, pixels::Color};

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

    'running: loop {

        // Clear screen with black
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();


        // Set draw color to white for the line
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw line
        canvas.draw_line(Point::new(375, 0), Point::new(375, 600)).unwrap();
        canvas.draw_line(Point::new(425, 0), Point::new(425, 600)).unwrap();


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

