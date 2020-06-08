use super::*;

const PC: u16 = 0xA00;
const PC_NEXT: u16 = 0xA02;

fn setup_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.pc = PC;
    cpu.v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
    cpu.vram[0][0] = 0xa8;
    cpu.vram[DISPLAY_HEIGHT-1][DISPLAY_WIDTH-1] = 0xf0;
    cpu
}


#[test]
fn test_init() {
    let cpu = Cpu::new();
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0x0);
    assert_eq!(cpu.delay_timer, 0x0);
    assert_eq!(cpu.sound_timer, 0x0);
    // TODO font data loading
}

#[test]
fn test_cls() {
    // 00E0 - CLS
    // Clear the display.
    let mut cpu = setup_cpu();
    cpu.execute(0x00E0);
    assert_eq!(cpu.vram[0][0], 0x0);
    assert_eq!(cpu.vram[DISPLAY_HEIGHT-1][DISPLAY_WIDTH-1], 0x0);
    assert_eq!(cpu.pc, PC_NEXT);
}

#[test]
fn test_ret() {
    // 00EE - RET Return from a subroutine.
    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    let mut cpu = setup_cpu();
    cpu.sp += 1;
    let final_cpu: u16 = 0x1234;
    cpu.stack[cpu.sp as usize] = final_cpu;
    cpu.execute(0x00EE);
    assert_eq!(cpu.pc, final_cpu);
    assert_eq!(cpu.sp, 0);
    cpu.sp = 15;
    let final_cpu2: u16 = 0xaced;
    cpu.stack[cpu.sp as usize] = final_cpu2;
    cpu.execute(0x00EE);
    assert_eq!(cpu.pc, final_cpu2);
    assert_eq!(cpu.sp, 14);
}

#[test]
fn test_jp() {
    // 1nnn - JP addr Jump to location nnn.  
    // The interpreter sets the program counter to nnn.
    let mut cpu = setup_cpu();
    cpu.execute(0x1ace);
    assert_eq!(cpu.pc, 0xace)
}


#[test]
fn test_call() {
    // 2nnn - CALL addr Call subroutine at nnn.
    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    let mut cpu = setup_cpu();
    cpu.execute(0x2ace);
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[1], PC);
    assert_eq!(cpu.pc, 0xace);
}

#[test]
fn test_skip_equal() {
    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.
    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    let mut cpu = setup_cpu();
    cpu.v[0] = 0x12;
    cpu.execute(0x3012);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
    cpu.pc = PC;
    cpu.v[0xE] = 0x12;
    cpu.execute(0x3E12);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
    cpu.pc = PC;
    cpu.v[0x7] = 0x11;  // negative test
    cpu.execute(0x3712);
    assert_eq!(cpu.pc, PC_NEXT);
}

#[test]
fn test_skip_not_equal() {
    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.
    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    let mut cpu = setup_cpu();
    cpu.v[0] = 0x12;
    cpu.execute(0x4012);
    assert_eq!(cpu.pc, PC_NEXT);
    cpu.pc = PC;
    cpu.v[0xE] = 0x12;
    cpu.execute(0x4E12);
    assert_eq!(cpu.pc, PC_NEXT);
    cpu.pc = PC;
    cpu.v[0x7] = 0x11;  // positive test
    cpu.execute(0x4712);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
}

#[test]
fn test_skip_equal_v() {
    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.
    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    let mut cpu = setup_cpu();
    cpu.v[0] = 0x12;
    cpu.v[1] = 0x12;
    cpu.v[2] = 0x7;
    cpu.execute(0x5010);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
    cpu.execute(0x5120);  // negative test
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE + OPCODE_SIZE);
}


#[test]
fn test_load_vx() {
    // 6xkk - LD Vx, byte
    // Set Vx = kk.
    // The interpreter puts the value kk into register Vx.
    let mut cpu = setup_cpu();
    cpu.execute(0x6012);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[0], 0x12);
}


#[test]
fn test_add() {
    // 7xkk - ADD Vx, byte
    // Set Vx = Vx + kk.
    // Adds the value kk to the value of register Vx, then stores the result in Vx.
    let mut cpu = setup_cpu();
    cpu.v[8] = 0x12;
    cpu.execute(0x7834);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[8], 0x12 + 0x34);
}

#[test]
fn test_ldx() {
    // 8xy0 - LD Vx, Vy
    // Set Vx = Vy.
    // Stores the value of register Vy in register Vx.
    let mut cpu = setup_cpu();
    cpu.v[4] = 0x12;
    cpu.v[8] = 0xfe;
    cpu.execute(0x8840);
    assert_eq!(cpu.v[8], 0x12);
    assert_eq!(cpu.pc, PC_NEXT);
}

/*
8xy1 - OR Vx, Vy
Set Vx = Vx OR Vy.
Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.


8xy2 - AND Vx, Vy
Set Vx = Vx AND Vy.
Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.


8xy3 - XOR Vx, Vy
Set Vx = Vx XOR Vy.
Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.


8xy4 - ADD Vx, Vy
Set Vx = Vx + Vy, set VF = carry.
The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.


8xy5 - SUB Vx, Vy
Set Vx = Vx - Vy, set VF = NOT borrow.
If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.


8xy6 - SHR Vx {, Vy}
Set Vx = Vx SHR 1.
If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.


8xy7 - SUBN Vx, Vy
Set Vx = Vy - Vx, set VF = NOT borrow.
If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.


8xyE - SHL Vx {, Vy}
Set Vx = Vx SHL 1.
If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.


9xy0 - SNE Vx, Vy
Skip next instruction if Vx != Vy.
The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.


Annn - LD I, addr
Set I = nnn.
The value of register I is set to nnn.


Bnnn - JP V0, addr
Jump to location nnn + V0.
The program counter is set to nnn plus the value of V0.


Cxkk - RND Vx, byte
Set Vx = random byte AND kk.
The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.


Dxyn - DRW Vx, Vy, nibble
Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.


Ex9E - SKP Vx
Skip next instruction if key with the value of Vx is pressed.
Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.


ExA1 - SKNP Vx
Skip next instruction if key with the value of Vx is not pressed.
Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.


Fx07 - LD Vx, DT
Set Vx = delay timer value.
The value of DT is placed into Vx.


Fx0A - LD Vx, K
Wait for a key press, store the value of the key in Vx.
All execution stops until a key is pressed, then the value of that key is stored in Vx.


Fx15 - LD DT, Vx
Set delay timer = Vx.
DT is set equal to the value of Vx.


Fx18 - LD ST, Vx
Set sound timer = Vx.
ST is set equal to the value of Vx.


Fx1E - ADD I, Vx
Set I = I + Vx.
The values of I and Vx are added, and the results are stored in I.


Fx29 - LD F, Vx
Set I = location of sprite for digit Vx.
The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.


Fx33 - LD B, Vx
Store BCD representation of Vx in memory locations I, I+1, and I+2.
The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.


Fx55 - LD [I], Vx
Store registers V0 through Vx in memory starting at location I.
The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.


Fx65 - LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.
The interpreter reads values from memory starting at location I into registers V0 through Vx.
*/
 
