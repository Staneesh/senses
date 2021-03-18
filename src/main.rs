extern crate sdl2;
extern crate gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("senses", 900, 700)
        .resizable()
        .build()
        .unwrap();
    
    let mut event_pump = sdl.event_pump().unwrap();
    
    'game_loop: loop { 
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'game_loop,
                _ => {},
            }
        }
    }
}
