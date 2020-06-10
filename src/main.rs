use std::env;
mod cpu;
mod cartridge;
mod input;
mod fonts;

use crate::cartridge::Cartridge;
use crate::input::Input;
use crate::cpu::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_file = &args[1];
    let cart = Cartridge::new(rom_file).expect("file not found");
    let input = Input::new();
    let mut cpu = Cpu::new();
    cpu.load_rom(&cart.rom);

    'game_loop: loop {
        // let time_before = instant::now();
        // handle events from SDL
        let keys = match input.poll() {
            Some(keys) => keys,
            None => break 'game_loop,
        };
        cpu.tick(keys)
    }
}
