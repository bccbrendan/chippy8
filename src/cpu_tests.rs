use super::*;

const PC: u16 = 0xA00;
const PC_NEXT: u16 = 0xA02;

fn setup_cpu() -> Cpu {
    let mut cpu = Cpu::new();
    cpu.pc = PC;
    cpu.v = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
    cpu
}


#[test]
fn test_init() {
    let cpu = Cpu::new();
    assert_eq!(cpu.pc, 0x200);
    assert_eq!(cpu.sp, 0x0);
    assert_eq!(cpu.delay_timer, 0x0);
    assert_eq!(cpu.sound_timer, 0x0);
    assert_eq!(cpu.ram[0], 0xF0); // first byte of 0
    assert_eq!(cpu.ram[1], 0x90); // second byte of 0
    assert_eq!(cpu.ram[HEX_DIGIT_DATA.len() - HEX_DIGIT_BYTE_LENGTH], 0xF0); // first byte of f
    assert_eq!(cpu.ram[1 + HEX_DIGIT_DATA.len() - HEX_DIGIT_BYTE_LENGTH], 0x80); // first byte of f
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
    assert_eq!(cpu.stack[1], PC + OPCODE_SIZE);
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

#[test]
fn test_or() {
    // 8xy1 - OR Vx, Vy - Set Vx = Vx OR Vy.
    // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    let mut cpu = setup_cpu();
    let x = 0x4;
    let y = 0x7;
    cpu.v[x] = 0xa5;
    cpu.v[y]  = 0x0f;
    cpu.execute(0x8471);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0xaf);
}


#[test]
fn test_and() {
    // 8xy2 - AND Vx, Vy - Set Vx = Vx AND Vy.
    // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    let mut cpu = setup_cpu();
    let x = 0x4;
    let y = 0x7;
    cpu.v[x] = 0xa5;
    cpu.v[y]  = 0x0F;
    cpu.execute(0x8472);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0x05);
}


#[test]
fn test_xor() {
    // 8xy3 - XOR Vx, Vy - Set Vx = Vx XOR Vy.
    // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    let mut cpu = setup_cpu();
    let x = 0x4;
    let y = 0x7;
    cpu.v[x] = 0xa0;
    cpu.v[y] = 0x15;
    cpu.execute(0x8473);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0xb5);
}


#[test]
fn test_add_vx_vy() {
    // 8xy4 - ADD Vx Vy - Set Vx = Vx + Vy, set VF = carry.
    // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    let mut cpu = setup_cpu();
    let x = 0x4;
    let y = 0x7;
    cpu.v[x] = 0x12;
    cpu.v[y] = 0x34;
    cpu.execute(0x8474);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0x12 + 0x34);
    assert_eq!(cpu.v[0xf], 0);
    cpu.v[x] = 0xF5;
    cpu.v[y] = 0x11;
    cpu.execute(0x8474);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
    assert_eq!(cpu.v[0xf], 1);
    assert_eq!(cpu.v[x], 0x06);
}


#[test]
fn test_sub() {
    // 8xy5 - SUB Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow.
    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    let x = 0x4;
    let y = 0x7;
    let mut cpu = setup_cpu();
    cpu.v[x] = 0xff;
    cpu.v[y] = 0x2;
    cpu.execute(0x8475);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[0xf], 1);
    assert_eq!(cpu.v[x], 0xfd);
    cpu.v[x] = 0x0;
    cpu.v[y] = 0x2;
    cpu.execute(0x8475);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
    assert_eq!(cpu.v[0xf], 0);
    assert_eq!(cpu.v[x], 0xfe);
}


#[test]
fn test_shr() {
    // 8xy6 - SHR Vx {, Vy} - Set Vx = Vx SHR 1.
    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0x62;
    cpu.v[0xf] = 0x11;
    cpu.execute(0x8476);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0x31);
    assert_eq!(cpu.v[0xf], 0x00);
    cpu.execute(0x8476);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
    assert_eq!(cpu.v[x], 0x18);
    assert_eq!(cpu.v[0xf], 0x01);
}


#[test]
fn test_subn() {
    // 8xy7 - SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    let mut cpu = setup_cpu();
    let x = 0x4;
    let y = 0x7;
    cpu.v[x] = 2;
    cpu.v[y] = 0xff;
    cpu.v[0xf] = 0xdb;
    cpu.execute(0x8477);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0xfd);
    assert_eq!(cpu.v[0xf], 1);
    cpu.v[x] = 1;
    cpu.v[y] = 0x0;
    cpu.v[0xf] = 0xdb;
    cpu.execute(0x8477);
    assert_eq!(cpu.v[x], 0xff);
    assert_eq!(cpu.v[0xf], 0);
}


#[test]
fn test_shl() {
    // 8xyE - SHL Vx {, Vy} - Set Vx = Vx SHL 1.
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0x12;
    cpu.v[0xf] = 0x9;
    cpu.execute(0x847E);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0x24);
    assert_eq!(cpu.v[0xf], 0);
    cpu.v[x] = 0x82;
    cpu.v[0xf] = 0x09;
    cpu.execute(0x847E);
    assert_eq!(cpu.v[x], 0x04);
    assert_eq!(cpu.v[0xf], 1);
}


#[test]
fn test_sne() {
    // 9xy0 - SNE Vx, Vy - Skip next instruction if Vx != Vy.
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    let mut cpu = setup_cpu();
    let x = 0x4;
    let y = 0x7;
    cpu.v[x] = 0x11;
    cpu.v[y] = 0x11;
    cpu.execute(0x9470);
    assert_eq!(cpu.pc, PC_NEXT);
    cpu.pc = PC;
    cpu.v[x] = 0x11;
    cpu.v[y] = 0x01;
    cpu.execute(0x9470);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
}


#[test]
fn test_ld_i() {
    // Annn - LD I, addr - Set I = nnn.
    // The value of register I is set to nnn.
    let mut cpu = setup_cpu();
    let nnn = 0x420;
    cpu.execute(0xA420);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.i, nnn);
}


#[test]
fn test_jpv() {
    // Bnnn - JPV0, addr - Jump to location nnn + V0.
    // The program counter is set to nnn plus the value of V0.
    let mut cpu = setup_cpu();
    cpu.v[0] = 0x24;
    cpu.execute(0xB420);
    assert_eq!(cpu.pc, 0x420 + 0x24);
}


#[test]
fn test_rnd() {
    // Cxkk - RND Vx, byte - Set Vx = random byte AND kk.
    // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
    let mut cpu = setup_cpu();
    let x = 4;
    cpu.execute(0xC400);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0x00);  // AND'd with 0
    cpu.execute(0xC40F);
    assert_eq!(cpu.v[x] & 0xF0, 0x00);
    cpu.execute(0xC4F0);
    assert_eq!(cpu.v[x] & 0x0F, 0x00);
}

#[test]
fn test_drw() {
    // Dxyn - DRW Vx, Vy, nibble - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
    let mut cpu = setup_cpu();
    cpu.i = 0;
    cpu.ram[0] = 0b11111111;
    cpu.ram[1] = 0b00000000;
    cpu.vram[0][0] = 1;
    cpu.vram[0][1] = 0;
    cpu.vram[1][0] = 1;
    cpu.vram[1][1] = 0;
    cpu.v[0] = 0;
    cpu.execute(0xd002);
    assert_eq!(cpu.vram[0][0], 0);
    assert_eq!(cpu.vram[0][1], 1);
    assert_eq!(cpu.vram[1][0], 1);
    assert_eq!(cpu.vram[1][1], 0);
    assert_eq!(cpu.v[0x0f], 1);
    assert!(cpu.vram_changed);
    assert_eq!(cpu.pc, PC_NEXT);
}


#[test]
fn test_draw_wrap_horizontal() {
    let mut cpu = setup_cpu();
    let x = DISPLAY_WIDTH - 4;
    cpu.i = 0;
    cpu.ram[0] = 0b11111111;
    cpu.v[0] = x as u8;
    cpu.v[1] = 0;
    cpu.execute(0xd011);
    assert_eq!(cpu.vram[0][x - 1], 0);
    assert_eq!(cpu.vram[0][x], 1);
    assert_eq!(cpu.vram[0][x + 1], 1);
    assert_eq!(cpu.vram[0][x + 2], 1);
    assert_eq!(cpu.vram[0][x + 3], 1);
    assert_eq!(cpu.vram[0][0], 1);
    assert_eq!(cpu.vram[0][1], 1);
    assert_eq!(cpu.vram[0][2], 1);
    assert_eq!(cpu.vram[0][3], 1);
    assert_eq!(cpu.vram[0][4], 0);
    assert_eq!(cpu.v[0x0f], 0);
}

#[test]
fn test_draw_wrap_vertical() {
    // Dxyn
    // DRW Vx, Vy, nibble
    let mut cpu = setup_cpu();
    let y = DISPLAY_HEIGHT - 1; // write on bottom row, a 2byte sprite
    cpu.i = 0;
    cpu.ram[0] = 0b11111111;
    cpu.ram[1] = 0b11111111;
    cpu.v[0] = 0;  // first column
    cpu.v[1] = y as u8;  // last row
    cpu.execute(0xd012);
    assert_eq!(cpu.vram[y][0], 1);
    assert_eq!(cpu.vram[0][0], 1);
    assert_eq!(cpu.v[0x0f], 0);
}

#[test]
fn test_skp() {
    // Ex9E - SKP Vx - Skip next instruction if key with the value of Vx is pressed.
    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0;
    cpu.ram[PC as usize] = 0xE4;
    cpu.ram[PC as usize + 1] = 0x9E;
    // all keys pressed except key0
    cpu.tick(&[false, true, true, true,
              true, true, true, true,
              true, true, true, true,
              true, true, true, true]);
    assert_eq!(cpu.pc, PC_NEXT);
    cpu.pc = PC;
    // key0 is only key pressed
    cpu.tick(&[true, false, false, false,
              false, false, false, false,
              false, false, false, false,
              false, false, false, false]);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
}


#[test]
fn test_sknp() {
    // ExA1 - SKNP Vx - Skip next instruction if key with the value of Vx is not pressed.
    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0;
    cpu.ram[PC as usize] = 0xE4;
    cpu.ram[PC as usize + 1] = 0xa1;
    // key0 is only key pressed
    cpu.tick(&[true, false, false, false,
              false, false, false, false,
              false, false, false, false,
              false, false, false, false]);
    assert_eq!(cpu.pc, PC_NEXT);

    cpu.pc = PC;
    // all keys pressed except key0
    cpu.tick(&[false, true, true, true,
              true, true, true, true,
              true, true, true, true,
              true, true, true, true]);
    assert_eq!(cpu.pc, PC_NEXT + OPCODE_SIZE);
}


#[test]
fn test_ld_vx_dt() {
    // Fx07 - LD Vx DT - Set Vx = delay timer value.
    // The value of DT is placed into Vx.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0x7;
    cpu.delay_timer = 0x42;
    cpu.execute(0xF407);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.v[x], 0x42);
}


#[test]
fn test_ld_vx_k() {
    // Fx0A - LD Vx K - Wait for a key press, store the value of the key in Vx.
    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
    let mut cpu = setup_cpu();
    cpu.execute(0xF40A);
    assert_eq!(cpu.pc, PC_NEXT);
    cpu.tick(&[false; 16]);
    assert_eq!(cpu.pc, PC_NEXT);
    // 0xF71E;  add I v[7], safe to exec
    cpu.ram[cpu.pc as usize] = 0xF7;
    cpu.ram[(cpu.pc + 1) as usize] = 0x1E;
    assert_eq!(cpu.pc, PC_NEXT);
    cpu.tick(&[false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]);
    assert_eq!(cpu.pc, PC_NEXT + 2);
    assert_eq!(cpu.v[4], 2);
}


#[test]
fn test_ld_dt_vx() {
    // Fx15 - LD DT Vx - Set delay timer = Vx.
    // DT is set equal to the value of Vx.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0xa5;
    cpu.execute(0xF415);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.delay_timer, 0xa5);
}


#[test]
fn test_ld_st_vx() {
    // Fx18 - LD ST Vx - Set sound timer = Vx.
    // ST is set equal to the value of Vx.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 0xa5;
    cpu.execute(0xF418);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.sound_timer, 0xa5);
}


#[test]
fn test_add_i_vx() {
    // Fx1E - ADD_I_Vx - Set I = I + Vx.
    // The values of I and Vx are added, and the results are stored in I.
    let mut cpu = setup_cpu();
    cpu.i = 0x123;
    let x = 0x4;
    cpu.v[x as usize] = 0x23;
    cpu.execute(0xF41E);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.i, 0x123 + 0x23);
}


#[test]
fn test_ld_f_vx() {
    // Fx29 - LD F Vx - Set I = location of sprite for digit Vx.
    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 7;
    cpu.execute(0xF429);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.i, 0 + (7 * 5));
    assert_eq!(cpu.i, HEX_DIGIT_ADDR_START + (7 * HEX_DIGIT_BYTE_LENGTH) as u16);
}


#[test]
fn test_ld_b_vx() {
    // Fx33 - LD_B_Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    let mut cpu = setup_cpu();
    let x = 0x4;
    cpu.v[x] = 123;
    cpu.i = 0x200;
    cpu.execute(0xF433);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.ram[cpu.i as usize], 1);
    assert_eq!(cpu.ram[cpu.i as usize + 1], 2);
    assert_eq!(cpu.ram[cpu.i as usize + 2], 3);
}


#[test]
fn test_ld_i_vx() {
    // Fx55 - LD [I] Vx - Store registers V0 through Vx in memory starting at location I.
    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    let mut cpu = setup_cpu();
    cpu.i = 0x200;
    cpu.execute(0xF455);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.ram[0x200], 0);
    assert_eq!(cpu.ram[0x201], 1);
    assert_eq!(cpu.ram[0x202], 2);
    assert_eq!(cpu.ram[0x203], 3);
    assert_eq!(cpu.ram[0x204], 4);
    assert_eq!(cpu.ram[0x205], 0);
}


#[test]
fn test_ld_vx_i() {
    // Fx65 - LD_Vx_I - Read registers V0 through Vx from memory starting at location I.
    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
    let mut cpu = setup_cpu();
    cpu.i = 0x20;
    cpu.ram[0x20] = 0xa;
    cpu.ram[0x21] = 0xc;
    cpu.ram[0x22] = 0xe;
    cpu.ram[0x23] = 0xd;
    cpu.ram[0x24] = 0x7;
    cpu.ram[0x25] = 0x9;  // shouldn't be copied
    cpu.v[5] = 0x0;
    // x = 4
    cpu.execute(0xF465);
    assert_eq!(cpu.pc, PC_NEXT);
    assert_eq!(cpu.i, 0x20);
    assert_eq!(cpu.v[0], 0xa);
    assert_eq!(cpu.v[1], 0xc);
    assert_eq!(cpu.v[2], 0xe);
    assert_eq!(cpu.v[3], 0xd);
    assert_eq!(cpu.v[4], 0x7);
    assert_eq!(cpu.v[5], 0x0);
}
 
