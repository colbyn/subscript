#![allow(dead_code, unused)]

use wasm_bindgen::prelude::*;

#[macro_use] extern crate subscript;

mod client;
mod server;


#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console!("started");
    client::setup();
    Ok(())
}

#[wasm_bindgen]
pub fn tick() {
    client::tick();
}






