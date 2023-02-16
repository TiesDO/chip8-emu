use crate::utils;

#[test]
fn memory_print_bpr_10() {
    let bytes: &[u8] = &[11, 12, 13, 14, 15, 16, 17];

    utils::debug::print_bytes(bytes, 10);
}

#[test]
fn memory_print_bpr_8() {
    let bytes: &[u8] = &[11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28];

    utils::debug::print_bytes(bytes, 8);
}
