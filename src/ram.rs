pub struct Chip8Ram{
    bytes: [u8; 4096],
}

impl Chip8Ram {
    // return all bytes
    pub fn get_bytes(&self) -> &[u8; 4096] {
        &self.bytes
    }

    pub fn write_bytes(&mut self, start: u8, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            self.bytes[usize::from(start) + i] = byte;
        };
    }

    pub fn new() -> Self {
        return Self { bytes: [0; 4096] }
    }
}
