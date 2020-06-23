mod cartridge;
mod input;
mod display;
mod fonts;
mod cpu;

use std::env;
use crate::cartridge::Cartridge;
use crate::input::Input;
use crate::display::Display;
use crate::cpu::Cpu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_file = &args[1];
    let cart = Cartridge::new(rom_file).expect("file not found");
    let mut cpu = Cpu::new();
    cpu.load_rom(&cart.rom);

    println!("initializing sdl2");
    let sdl = sdl2::init().unwrap();
    let mut input = Input::new();
    println!("creating window");
    let scale_xy = 16;
    let mut display = Display::new(&sdl, scale_xy);
    let mut event_pump = sdl.event_pump().unwrap();
    println!("starting game loop");
    'game_loop: loop {
        // handle events like key presses and window resizing/closing
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                }
                Event::KeyDown {..} => { input.keydown(event) }
                Event::KeyUp {..} => { input.keyup(event) }
                // TODO resize with WindowEvent::Resized(i32, i32)
                _ => {}
            }
        }
        // let time_before = instant::now();
        let keys = input.keys_pressed();
        let output = cpu.tick(keys);
        if output.vram_changed {
            // println!("drawing");
            display.draw(output.vram);
        }
    }
}
