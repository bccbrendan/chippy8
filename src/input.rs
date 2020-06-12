use sdl2::EventPump;
use sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: EventPump,
}


impl Input {
    pub fn new(sdl: &sdl2::Sdl) -> Self {
        Input {
            event_pump: sdl.event_pump().unwrap(),
        }
    }

    // TODO 
    pub fn poll(&mut self) -> Option<[bool; 16]> {
        let keys = [false; 16];
        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::KeyDown {..} => { Some([false; 16]); },
                Event::Quit {..}|
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return None
                }
                _ => { }
            }
        }
        Some(keys)
    }
}
