// reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
use rand::Rng;
use crate::fonts::*;

const RESET_VECTOR: u16 = 0x200;
const RAM_LENGTH: usize = 0x1000;
const OPCODE_SIZE: u16 = 2;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

enum InstructionPointer {
    Inc, // just run the next instruction
    Jump(u16), // set PC to the given addr
    Skip, // skip one PC instruction
}

pub struct Cpu {
    pc: u16,
    v: [u8; 16],
    sp: u8,
    stack: [u16; 16],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    ram: [u8; 0x1000],
    vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    awaiting_keypress: bool,
    first_key_pressed_register: usize,
    keys_pressed: [bool; 16],
    vram_changed: bool,
}

impl Cpu {

    pub fn new() -> Self {
        let mut ram = [0; RAM_LENGTH];
        ram[..HEX_DIGIT_DATA.len()].copy_from_slice(&HEX_DIGIT_DATA);
        Cpu {
            pc: RESET_VECTOR,
            v: [0; 16],
            sp: 0,
            stack: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            ram: ram,
            vram: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            awaiting_keypress: false,
            first_key_pressed_register: 0,
            keys_pressed: [false; 16],
            vram_changed: false,
        }
    }

    pub fn tick(&mut self, keys_pressed: [bool; 16]) {
        self.keys_pressed = keys_pressed;
        if self.awaiting_keypress {
            for i in 0..keys_pressed.len() {
                if keys_pressed[i] {
                    self.v[self.first_key_pressed_register] = i as u8;
                    self.awaiting_keypress = false;
                    break;
                }
            }
        }
        if !self.awaiting_keypress {
            let op = self.fetch();
            self.execute(op)
        }
    }

    pub fn fetch(&self) -> u16 {
        let addr = self.pc as usize;
        (self.ram[addr] as u16) << 8 | (self.ram[addr+1] as u16)
    }

    pub fn execute(&mut self, opcode: u16) {
        let nibbles = (
            (opcode >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8
        );
        let nnn = (opcode & 0x0FFF) as u16;
        let kk =  (opcode & 0x00FF) as u8;
        let x =  ((opcode & 0x0F00) >> 8) as usize;
        let y =  ((opcode & 0x00F0) >> 4) as usize;
        let n =   (opcode & 0x000F) as usize;
        let next_ip = match nibbles {
            // (0x0,   _,   _,   _)  => panic!("SYS addr - ignored."),
            (0x0, 0x0, 0xE, 0x0) => self.op_cls(),
            (0x0, 0x0, 0xE, 0xE) => self.op_ret(),
            (0x1,   _,   _,   _) => self.op_jp(nnn),
            (0x2,   _,   _,   _) => self.op_call(nnn),
            (0x3,   _,   _,   _) => self.op_se(x, kk),
            (0x4,   _,   _,   _) => self.op_sne(x, kk),
            (0x5,   _,   _,   _) => self.op_se_vxy(x, y),
            (0x6,   _,   _,   _) => self.op_ldx_b(x, kk),
            (0x7,   _,   _,   _) => self.op_add(x, kk),
            (0x8,   _,   _, 0x0) => self.op_ldx_y(x, y),
            (0x8,   _,   _, 0x1) => self.op_or(x, y),
            (0x8,   _,   _, 0x2) => self.op_and(x, y),
            (0x8,   _,   _, 0x3) => self.op_xor(x, y),
            (0x8,   _,   _, 0x4) => self.op_add_xy(x, y),
            (0x8,   _,   _, 0x5) => self.op_sub_xy(x, y),
            (0x8,   _,   _, 0x6) => self.op_shr(x, y),
            (0x8,   _,   _, 0x7) => self.op_subn(x, y),
            (0x8,   _,   _, 0xE) => self.op_shl(x, y),
            (0x9,   _,   _, 0x0) => self.op_sne_xy(x, y),
            (0xa,   _,   _,   _) => self.op_ld_i(nnn),
            (0xb,   _,   _,   _) => self.op_jpv(nnn),
            (0xc,   _,   _,   _) => self.op_rnd(x, kk),
            (0xd,   _,   _,   _) => self.op_drw(x, y, n),
            (0xe,   _, 0x9, 0xe) => self.op_skp(x),
            (0xe,   _, 0xa, 0x1) => self.op_sknp(x),
            (0xf,   _, 0x0, 0x7) => self.op_ld_vx_dt(x),
            (0xf,   _, 0x0, 0xa) => self.op_ld_vx_k(x),
            (0xf,   _, 0x1, 0x5) => self.op_ld_dt_vx(x),
            (0xf,   _, 0x1, 0x8) => self.op_ld_st_vx(x),
            (0xf,   _, 0x2, 0x9) => self.op_ld_f_vx(x),
            (0xf,   _, 0x3, 0x3) => self.op_ld_b_vx(x),
            (0xf,   _, 0x5, 0x5) => self.op_ld_i_vx(x),
            (0xf,   _, 0x6, 0x5) => self.op_ld_vx_i(x),
            (0xf,   _,   _, 0xe) => self.op_add_i_vx(x),

            _ => panic!("Unrecognized opcode: {opcode}"),
        };
        match next_ip {
            InstructionPointer::Inc => self.pc += OPCODE_SIZE,
            InstructionPointer::Jump(addr) => self.pc = addr,
            InstructionPointer::Skip => self.pc += OPCODE_SIZE * 2,
        }
    }

    /*
    pub fn 
    0nnn - SYS addr
    Jump to a machine code routine at nnn.
    This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
    */

    // 00E0 - CLS - Clear the display.
    fn op_cls(&mut self) -> InstructionPointer {
        self.vram = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        InstructionPointer::Inc
    }

    // 00EE - RET - Return from a subroutine.
    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn op_ret(&mut self) -> InstructionPointer {
        let next_pc = self.stack[self.sp as usize];
        self.sp -= 1;
        InstructionPointer::Jump(next_pc)
    }

    // 1nnn - JP addr Jump to location nnn. The interpreter sets the program counter to nnn.
    fn op_jp(&mut self, nnn: u16) -> InstructionPointer {
        InstructionPointer::Jump(nnn)
    }


    // 2nnn - CALL addr Call subroutine at nnn.
    // The interpreter increments the stack pointer,
    // then puts the current PC on the top of the stack. 
    // The PC is then set to nnn.
    fn op_call(&mut self, nnn: u16) -> InstructionPointer {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        InstructionPointer::Jump(nnn)
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    fn op_se(&mut self, x: usize, kk: u8) -> InstructionPointer {
        if self.v[x] == kk {
            InstructionPointer::Skip
        } else {
            InstructionPointer::Inc
        }
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_sne(&mut self, x: usize, kk: u8) -> InstructionPointer {
        if self.v[x] != kk {
            InstructionPointer::Skip
        } else {
            InstructionPointer::Inc
        }
    }

    // 5xy0 - SE Vx, Vy -  Skip next instruction if Vx = Vy.
    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_se_vxy(&mut self, x: usize, y: usize) -> InstructionPointer {
        if self.v[x] == self.v[y] {
            InstructionPointer::Skip
        } else {
            InstructionPointer::Inc
        }
    }

    // 6xkk - LD Vx, byte - Set Vx = kk.
    // The interpreter puts the value kk into register Vx.
    fn op_ldx_b(&mut self, x: usize, kk: u8) -> InstructionPointer {
        self.v[x] = kk;
        InstructionPointer::Inc
    }


    // 7xkk - ADD Vx, byte - Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    fn op_add(&mut self, x: usize, kk: u8) -> InstructionPointer {
        self.v[x] += kk;
        InstructionPointer::Inc
    }

    // 8xy0 - LD Vx, Vy - Set Vx = Vy.
    // Stores the value of register Vy in register Vx.
    fn op_ldx_y(&mut self, x: usize, y: usize) -> InstructionPointer {
        self.v[x] = self.v[y];
        InstructionPointer::Inc
    }

    // 8xy1 - OR Vx, Vy. Set Vx = Vx OR Vy.
    fn op_or(&mut self, x: usize, y: usize) -> InstructionPointer {
        self.v[x] = self.v[x] | self.v[y];
        InstructionPointer::Inc
    }


    // 8xy2 - AND Vx, Vy
    // Set Vx = Vx AND Vy.
    fn op_and(&mut self, x: usize, y: usize) -> InstructionPointer {
        self.v[x] = self.v[x] & self.v[y];
        InstructionPointer::Inc
    }
 

    // 8xy3 - XOR Vx, Vy - Set Vx = Vx XOR Vy.
    fn op_xor(&mut self, x: usize, y: usize) -> InstructionPointer {
        self.v[x] = self.v[x] ^ self.v[y];
        InstructionPointer::Inc
    }
 

    // 8xy4 - ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry.
    // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) 
    // VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn op_add_xy(&mut self, x: usize, y: usize) -> InstructionPointer {
        let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = sum;
        self.v[0xf] = if carry { 1 } else { 0 };
        InstructionPointer::Inc
    }
 

    // 8xy5 - SUB Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow.
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn op_sub_xy(&mut self, x: usize, y: usize) -> InstructionPointer {
        let (diff, borrow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = diff;
        self.v[0xf] = if !borrow { 1 } else { 0 };
        InstructionPointer::Inc
    }
 

    // 8xy6 - SHR Vx {, Vy} - Set Vx = Vx SHR 1.
    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn op_shr(&mut self, x: usize, _: usize) -> InstructionPointer {
        self.v[0xf] = if self.v[x] & 0x01 != 0 { 1 } else { 0 };
        self.v[x] = self.v[x] >> 1;
        InstructionPointer::Inc
    }
    


    // 8xy7 - SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn op_subn(&mut self, x: usize, y: usize) -> InstructionPointer {
        let (diff, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[0xf] = if !overflow { 1 } else { 0 };
        self.v[x] = diff;
        InstructionPointer::Inc
    }

    // 8xyE - SHL Vx {, Vy} - Set Vx = Vx SHL 1.
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn op_shl(&mut self, x: usize, _y: usize) -> InstructionPointer {
        self.v[0xf] = if self.v[x] & 0x80 != 0 { 1 } else { 0 };
        self.v[x] = self.v[x] << 1;
        InstructionPointer::Inc
    }


    // 9xy0 - SNE Vx, Vy - Skip next instruction if Vx != Vy.
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    fn op_sne_xy(&mut self, x: usize, y: usize) -> InstructionPointer {
        if self.v[x] == self.v[y] {
            InstructionPointer::Inc
        } else {
            InstructionPointer::Skip
        }
    }


    // Annn - LD I, addr - Set I = nnn.
    // The value of register I is set to nnn.
    fn op_ld_i(&mut self, nnn: u16) -> InstructionPointer {
        self.i = nnn;
        InstructionPointer::Inc
    }


    // Bnnn - JP V0, addr - Jump to location nnn + V0.
    // The program counter is set to nnn plus the value of V0.
    fn op_jpv(&mut self, nnn: u16) -> InstructionPointer {
        InstructionPointer::Jump(nnn + self.v[0] as u16)
    }

    // Cxkk - RND Vx, byte - Set Vx = random byte AND kk.
    // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn op_rnd(&mut self, x: usize, kk: u8) -> InstructionPointer {
        let r: u8 = rand::thread_rng().gen();
        self.v[x] = kk & r;
        InstructionPointer::Inc
    }


    // Dxyn - DRW Vx, Vy, nibble - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    // The interpreter reads n bytes from memory, 
    // starting at the address stored in I. 
    // These bytes are then displayed as sprites on screen 
    // at coordinates (Vx, Vy). 
    // Sprites are XORed onto the existing screen. 
    // If this causes any pixels to be erased, VF is set to 1,
    // otherwise it is set to 0. 
    // If the sprite is positioned so part of it is outside 
    // the coordinates of the display, it wraps around to 
    // the opposite side of the screen. 
    //
    // vram should be laid out as a 64x32 monochrome pixel display
    //               x tracks columns
    //  +--------------------------------------------+
    // y| (0,0)                               (63,0) |
    // =|                                            |
    // r|                                            |
    // o|                                            |
    // w| (0,31)                             (63,31) |
    //  +--------------------------------------------+
    fn op_drw(&mut self, x: usize, y: usize, n: usize) -> InstructionPointer {
        self.v[0xf] = 0;
        for byte in 0..n {
            let row = (self.v[y] as usize + byte) % DISPLAY_HEIGHT;
            let pixel_byte = self.ram[self.i as usize + byte];
            for bit in 0..8 {
                let column = (self.v[x] as usize + bit) % DISPLAY_WIDTH;
                let pixel_data = (pixel_byte >> (7 - bit)) & 0x1;
                let current_data = self.vram[row][column];
                if (current_data & pixel_data) != 0 {
                    self.v[0xf] = 0x1;
                }
                self.vram[row][column] = current_data ^ pixel_data;
            }
        }
        self.vram_changed = true;
        InstructionPointer::Inc
    }



    // Ex9E - SKP Vx - Skip next instruction if key with the value of Vx is pressed.
    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    fn op_skp(&mut self, x: usize) -> InstructionPointer {
        if self.keys_pressed[self.v[x] as usize] {
            InstructionPointer::Skip
        } else {
            InstructionPointer::Inc
        }
    }

    // ExA1 - SKNP Vx - Skip next instruction if key with the value of Vx is not pressed.
    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    fn op_sknp(&mut self, x: usize) -> InstructionPointer {
        if !self.keys_pressed[self.v[x] as usize] {
            InstructionPointer::Skip
        } else {
            InstructionPointer::Inc
        }
    }

    // Fx07 - LD Vx, DT - Set Vx = delay timer value.
    // The value of DT is placed into Vx.
    fn op_ld_vx_dt(&mut self, x: usize) -> InstructionPointer {
        self.v[x] = self.delay_timer;
        InstructionPointer::Inc
    }

    // Fx0A - LD Vx, K - Wait for a key press, store the value of the key in Vx.
    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
    fn op_ld_vx_k(&mut self, x: usize) -> InstructionPointer {
        self.first_key_pressed_register = x;
        self.awaiting_keypress = true;
        InstructionPointer::Inc
    }

    // Fx15 - LD DT, Vx - Set delay timer = Vx.
    // DT is set equal to the value of Vx.
    fn op_ld_dt_vx(&mut self, x: usize) -> InstructionPointer {
        self.delay_timer = self.v[x];
        InstructionPointer::Inc
    }

    // Fx18 - LD ST, Vx - Set sound timer = Vx.
    fn op_ld_st_vx(&mut self, x: usize) -> InstructionPointer {
        self.sound_timer = self.v[x];
        InstructionPointer::Inc
    }

    // Fx1E - ADD I, Vx - Set I = I + Vx.
    // The values of I and Vx are added, and the results are stored in I.
    fn op_add_i_vx(&mut self, x: usize) -> InstructionPointer {
        self.i += self.v[x] as u16;
        InstructionPointer::Inc
    }

    // Fx29 - LD F, Vx - Set I = location of sprite for digit Vx.
    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
    // See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    fn op_ld_f_vx(&mut self, x: usize) -> InstructionPointer {
        self.i = HEX_DIGIT_ADDR_START + self.v[x] as u16 * HEX_DIGIT_BYTE_LENGTH as u16;
        InstructionPointer::Inc
    }

    // Fx33 - LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    fn op_ld_b_vx(&mut self, x: usize) -> InstructionPointer {
        let hundreds = self.v[x] / 100;
        let tens = (self.v[x] % 100) / 10;
        let ones = self.v[x] % 10;
        self.ram[self.i as usize] = hundreds;
        self.ram[self.i as usize + 1] = tens;
        self.ram[self.i as usize + 2] = ones;
        InstructionPointer::Inc
    }


    // Fx55 - LD [I], Vx - Store registers V0 through Vx in memory starting at location I.
    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    fn op_ld_i_vx(&mut self, x: usize) -> InstructionPointer {
        for i in 0..x+1 {
            self.ram[self.i as usize + i] = self.v[i];
        }
        InstructionPointer::Inc
    }


    // Fx65 - LD Vx, [I] - Read registers V0 through Vx from memory starting at location I.
    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn op_ld_vx_i(&mut self, x: usize) -> InstructionPointer {
        for i in 0..x+1 {
            self.v[i] = self.ram[self.i as usize + i];
        }
        InstructionPointer::Inc
    }
 
}

#[cfg(test)]
#[path = "./cpu_tests.rs"]
mod cpu_tests;
