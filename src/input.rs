use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct Input {
    keys_pressed: [bool; 16],
}


impl Input {
    pub fn new() -> Self {
        Input {
            keys_pressed: [false; 16],
        }
    }

    pub fn keys_pressed(&self) -> &[bool; 16] {
        &self.keys_pressed
    }

    pub fn keydown(&mut self, event: sdl2::event::Event) {
        /* keyboard layout:
           1 2 3 c
           4 5 6 d
           7 8 9 e
           a 0 b f
        */
        match event {
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { self.keys_pressed[1] = true },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { self.keys_pressed[2] = true },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { self.keys_pressed[3] = true },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { self.keys_pressed[0xc] = true }
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => { self.keys_pressed[4] = true }
            Event::KeyDown { keycode: Some(Keycode::W), .. } => { self.keys_pressed[5] = true }
            Event::KeyDown { keycode: Some(Keycode::E), .. } => { self.keys_pressed[6] = true }
            Event::KeyDown { keycode: Some(Keycode::R), .. } => { self.keys_pressed[0xd] = true }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => { self.keys_pressed[7] = true }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => { self.keys_pressed[8] = true }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => { self.keys_pressed[9] = true }
            Event::KeyDown { keycode: Some(Keycode::F), .. } => { self.keys_pressed[0xe] = true }
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => { self.keys_pressed[0xa] = true }
            Event::KeyDown { keycode: Some(Keycode::X), .. } => { self.keys_pressed[0] = true }
            Event::KeyDown { keycode: Some(Keycode::C), .. } => { self.keys_pressed[0xb] = true }
            Event::KeyDown { keycode: Some(Keycode::V), .. } => { self.keys_pressed[0xf] = true }
            _ => {},
        }
    }

    pub fn keyup(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { self.keys_pressed[1] = false },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { self.keys_pressed[2] = false },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { self.keys_pressed[3] = false },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { self.keys_pressed[0xc] = false }
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => { self.keys_pressed[4] = false }
            Event::KeyDown { keycode: Some(Keycode::W), .. } => { self.keys_pressed[5] = false }
            Event::KeyDown { keycode: Some(Keycode::E), .. } => { self.keys_pressed[6] = false }
            Event::KeyDown { keycode: Some(Keycode::R), .. } => { self.keys_pressed[0xd] = false }
            Event::KeyDown { keycode: Some(Keycode::A), .. } => { self.keys_pressed[7] = false }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => { self.keys_pressed[8] = false }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => { self.keys_pressed[9] = false }
            Event::KeyDown { keycode: Some(Keycode::F), .. } => { self.keys_pressed[0xe] = false }
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => { self.keys_pressed[0xa] = false }
            Event::KeyDown { keycode: Some(Keycode::X), .. } => { self.keys_pressed[0] = false }
            Event::KeyDown { keycode: Some(Keycode::C), .. } => { self.keys_pressed[0xb] = false }
            Event::KeyDown { keycode: Some(Keycode::V), .. } => { self.keys_pressed[0xf] = false }
            _ => {},
        }
     }
}

