#![allow(dead_code, unused)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[macro_use]
pub mod ui;
pub mod dev;
pub mod browser;


#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    dev::run();
    Ok(())
}




