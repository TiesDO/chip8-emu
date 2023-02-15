use std::cmp::{self, Ordering};

pub struct Chip8Ram {
    bytes: [u8; 4096],
}

impl Chip8Ram {
    pub fn get_bytes(&self) -> &[u8; 4096] {
        &self.bytes
    }

    pub fn write_bytes(&mut self, start: u16, bytes: &[u8]) {
        for (i, &byte) in bytes.iter().enumerate() {
            self.bytes[usize::from(start) + i] = byte;
        };
    }

    pub fn compare_bytes(&self, start: u16, bytes: &[u8]) -> cmp::Ordering {
        // loop over both arrays (zip) and compare each element
        let start: usize = usize::from(start);
        let end: usize = start + bytes.len();
        
        // loop over both arrays
        for (self_i, comp_i) in self.bytes[start .. end].iter().zip(bytes.iter()) {
            match self_i.cmp(comp_i) {
                Ordering::Equal => continue,
                // return any result that is not Equal
                ord => return ord
            }
        }

        // compare the lengths as a return result if everything was Equal
        bytes.len().cmp(&(end - start))
    }

    pub fn new() -> Self {
        return Self { bytes: [0; 4096] }
    }
}
