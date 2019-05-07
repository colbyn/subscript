#![allow(dead_code, unused)]

use wasm_bindgen::prelude::*;
#[macro_use]
extern crate lazy_static;

pub mod browser;
#[macro_use]
pub mod tree;
#[macro_use]
pub mod process;
#[macro_use]
pub mod effect;
pub mod extras;

pub mod dev;


#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    dev::main();
    Ok(())
}




