pub const HEX_DIGIT_BYTE_LENGTH: usize = 5;
pub const HEX_DIGIT_ADDR_START: u16 = 0x000;
#[allow(dead_code)]
pub const HEX_DIGIT_ADDR_END: u16 = 0x1FF;
pub const HEX_DIGIT_DATA: [u8; 80] = [
    // "0" Binary Hex
    // ****
    // *  *
    // *  *
    // *  *
    // ****
    0xF0,
    0x90,
    0x90,
    0x90,
    0xF0,
    // "1" Binary Hex
    //   * 
    //  ** 
    //   * 
    //   * 
    //  ***
    0x20,
    0x60,
    0x20,
    0x20,
    0x70,
    // "2" Binary Hex
    // ****
    //    *
    // ****
    // *   
    // ****
    0xF0,
    0x10,
    0xF0,
    0x80,
    0xF0,
    // "3" Binary Hex
    // ****
    //    *
    // ****
    //    *
    // ****
    0xF0,
    0x10,
    0xF0,
    0x10,
    0xF0,
    // "4" Binary Hex
    // *  *
    // *  *
    // ****
    //    *
    //    *
    0x90,
    0x90,
    0xF0,
    0x10,
    0x10,
    // "5" Binary Hex
    // ****
    // *   
    // ****
    //    *
    // ****
    0xF0,
    0x80,
    0xF0,
    0x10,
    0xF0,
    // "6" Binary Hex
    // ****
    // *   
    // ****
    // *  *
    // ****
    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,
    // "7" Binary Hex
    // ****
    //    *
    //   * 
    //  *  
    //  *  
    0xF0,
    0x10,
    0x20,
    0x40,
    0x40,
    // "8" Binary Hex
    // ****
    // *  *
    // ****
    // *  *
    // ****
    0xF0,
    0x90,
    0xF0,
    0x90,
    0xF0,
    // "9" Binary Hex
    // ****
    // *  *
    // ****
    //    *
    // ****
    0xF0,
    0x90,
    0xF0,
    0x10,
    0xF0,
    // "A" Binary Hex
    // ****
    // *  *
    // ****
    // *  *
    // *  *
    0xF0,
    0x90,
    0xF0,
    0x90,
    0x90,
    // "B" Binary Hex
    // *** 
    // *  *
    // *** 
    // *  *
    // *** 
    0xE0,
    0x90,
    0xE0,
    0x90,
    0xE0,
    // "C" Binary Hex
    // ****
    // *   
    // *   
    // *   
    // ****
    0xF0,
    0x80,
    0x80,
    0x80,
    0xF0,
    // "D" Binary Hex
    // *** 
    // *  *
    // *  *
    // *  *
    // *** 
    0xE0,
    0x90,
    0x90,
    0x90,
    0xE0,
    // "E" Binary Hex
    // ****
    // *   
    // ****
    // *   
    // ****
    0xF0,
    0x80,
    0xF0,
    0x80,
    0xF0,
    // "F" Binary Hex
    // ****
    // *   
    // ****
    // *   
    // *   
    0xF0,
    0x80,
    0xF0,
    0x80,
    0x80,
];
