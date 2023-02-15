#![allow(unused_variables)]
use wasm_bindgen::prelude::*;
use ram::Chip8Ram;

mod ram;

#[wasm_bindgen(start)]
fn start() {
    console_log("testing");
}

fn console_log(str: &str) {
    use web_sys::console;

    console::log_1(&str.into());
}

#[cfg(test)]
mod ram_tests;

