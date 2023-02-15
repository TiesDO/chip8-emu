mod tests {
    use crate::{ram::Chip8Ram, utils::font};

    fn get_type<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

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

    #[test]
    fn compare_start() {
        let mut ram = Chip8Ram::new();

        let to_write = [4];
        ram.write_bytes(0, &to_write);

        let result = ram.compare_bytes(0, &to_write);

        assert_eq!(result, std::cmp::Ordering::Equal);
    }

    #[test]
    fn compare_offset() {
        let mut ram = Chip8Ram::new();

        let to_write = [4,3,2,1];
        let offset: u16 = 200;

        ram.write_bytes(offset, &to_write);

        let result = ram.compare_bytes(offset, &to_write);
        
        assert_eq!(result, std::cmp::Ordering::Equal);
    }

    #[test]
    fn write_font() {
        let mut ram = Chip8Ram::new();

        // write the font to the ram
        font::write_font(&mut ram);

        // compare the result
        let mut i: u16 = 0;

        while i < 16 {
            let result = ram.compare_bytes(font::CHAR_OFFSET + i * 5, font::CHAR_MAP[usize::from(i)]);
            
            assert_eq!(result, std::cmp::Ordering::Equal);

            i += 1;
        }
    }
}
