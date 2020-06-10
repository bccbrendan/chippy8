use std::fs::File;
use std::io::prelude::*;
use std::io;

pub const MAX_ROM_SIZE: usize = 0xE00;

pub struct Cartridge {
    pub rom: Vec<u8>,
}


impl Cartridge {
    pub fn new(rom_file_path: &str) -> io::Result<Self> {
        let mut rom_file = File::open(rom_file_path).expect(&format!("{} not found", rom_file_path));
        let mut buffer = Vec::new();
        rom_file.read_to_end(&mut buffer)?;
        Ok(Cartridge {
            rom: buffer,
        })
    }
}
