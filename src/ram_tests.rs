use super::*;

mod tests {
    use crate::ram::Chip8Ram;

    #[test]
    fn write_ram() {

        // create a new mutable ram object
        let mut ram = Chip8Ram::new();

        // bytes to write
        let to_write = [4, 3, 2, 1];

        // write bytes to ram
        ram.write_bytes(0, &to_write);

        // read bytes back
        let read: &[u8] = &ram.get_bytes()[0 .. 5];

        // check if the bytes are equal
        assert_eq!(read, [4, 3, 2, 1, 0]);
    }

    #[test]
    fn write_ram_fail() {
        let mut ram = Chip8Ram::new();

        let to_write = [4, 3, 2, 1];
        ram.write_bytes(0, &to_write);

        let read = &ram.get_bytes()[0 .. 5];

        assert_ne!(read, [0; 5]);
    }
}
