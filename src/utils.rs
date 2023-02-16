pub mod font {
    use super::super::ram::Chip8Ram;
    
    pub const CHAR_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
    pub const CHAR_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
    pub const CHAR_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
    pub const CHAR_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
    pub const CHAR_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
    pub const CHAR_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
    pub const CHAR_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
    pub const CHAR_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
    pub const CHAR_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
    pub const CHAR_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
    pub const CHAR_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
    pub const CHAR_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
    pub const CHAR_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
    pub const CHAR_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
    pub const CHAR_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
    pub const CHAR_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

    pub const CHAR_MAP: [&[u8; 5]; 16] = [&CHAR_0, &CHAR_1, &CHAR_2, &CHAR_3, &CHAR_4, &CHAR_5, &CHAR_6, &CHAR_7, &CHAR_8, &CHAR_9, &CHAR_A, &CHAR_B, &CHAR_C, &CHAR_D, &CHAR_E, &CHAR_F];

    pub const CHAR_OFFSET: u16 = 80;

    pub fn write_font(ram: &mut Chip8Ram) {
        let mut i: u16 = 0;
        
        // write all the characters to the given ram
        while i < 16 {
            let offset: u16 = CHAR_OFFSET + i * 5;
            ram.write_bytes(offset, CHAR_MAP[usize::from(i)]);

            let read = &ram.get_bytes()[usize::from(offset) .. usize::from(offset) + 5];

            i += 1;
        }
    }

}

pub mod debug {

    pub fn print_bytes(bytes: &[u8], bytes_per_row: u8) {
        let rows = bytes.len() / usize::from(bytes_per_row);
        let remainder = bytes.len() - usize::from(bytes_per_row) * rows;
        let mut i:usize = 0;

        let mut header_i : u8 = 0;
        while header_i < bytes_per_row {
            header_i += 1;
            print!("{:01$} ", header_i, 4);
        }

        print!("\n");

        while i < rows + 1 {
            let mut bpr = usize::from(bytes_per_row);

            if i == rows {
               bpr = remainder; 
            }

            let start = i * bpr;
            let end = start + bpr;
            
            for (j, &byte) in bytes[start .. end].iter().enumerate() {
                // print single byte in row
                print!("{:#04x} ", byte);
            }
            
            print!("\n");

            i += 1;
        }
    }
}
