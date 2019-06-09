#![allow(dead_code, unused)]

#[macro_use]
pub mod backend;
pub mod model;
pub mod view;
pub mod program;
pub mod dev;

use wasm_bindgen::prelude::*;
use ss_web_utils::js::{self, console};

#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console::log("started");
    dev::alpha::setup();
    Ok(())
}

#[wasm_bindgen]
pub fn tick() {
	dev::alpha::tick();
}






