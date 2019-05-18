#![allow(dead_code, unused)]

pub mod dev;

use wasm_bindgen::prelude::*;
use web_utils::js::{self, console};

#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console::log("running...");
    dev::main();
    Ok(())
}






