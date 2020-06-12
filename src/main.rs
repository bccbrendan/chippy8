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

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_file = &args[1];
    let cart = Cartridge::new(rom_file).expect("file not found");
    let mut cpu = Cpu::new();
    cpu.load_rom(&cart.rom);

    println!("initializing sdl2");
    let sdl = sdl2::init().unwrap();
    let mut input = Input::new(&sdl);
    println!("creating window");
    let scale_xy = 16;
    let mut display = Display::new(&sdl, scale_xy);
    println!("starting game loop");
    'game_loop: loop {
        // let time_before = instant::now();
        // handle events from SDL
        let keys = match input.poll() {
            Some(keys) => keys,
            None => break 'game_loop,
        };
        println!("ticking");
        let output = cpu.tick(keys);
        if output.vram_changed {
            println!("drawing");
            display.draw(output.vram);
        }
    }
}
