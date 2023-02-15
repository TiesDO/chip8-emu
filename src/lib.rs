#![allow(unused_variables)]
use utils::font;
use wasm_bindgen::prelude::*;
use ram::Chip8Ram;

mod ram;
mod utils;

#[wasm_bindgen(start)]
fn start() {
    console_log("creating virtual ram");
    let mut ram = Chip8Ram::new();

    console_log("writing font into memory");
    font::write_font(&mut ram)
}

fn console_log(str: &str) {
    use web_sys::console;

    console::log_1(&str.into());
}

#[cfg(test)]
mod ram_tests;

