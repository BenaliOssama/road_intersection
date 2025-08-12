use sdl2::{event::Event, keyboard::Keycode, init};

fn main() {
    let sdl_context = init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _window = video_subsystem
        .window("Road Intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
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

