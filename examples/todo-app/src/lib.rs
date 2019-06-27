#![allow(dead_code, unused)]

use wasm_bindgen::prelude::*;

#[macro_use] extern crate subscript;

mod app;
pub mod ui_utils;


#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console!("started");
    // dev::cms_app::client::setup();
    // dev::todo_app::setup();
    Ok(())
}

#[wasm_bindgen]
pub fn tick() {
    // dev::cms_app::client::tick();
    // dev::todo_app::tick();
}











