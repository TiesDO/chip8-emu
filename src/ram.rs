pub struct Chip8Ram{
    // the memory itself
    pub bytes: [u8; 4096],
}

impl Default for Chip8Ram {
    fn default() -> Chip8Ram {
        Chip8Ram {
            bytes: [0; 4096]
        }
    }
}
