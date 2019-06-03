#![allow(dead_code, unused)]
#[macro_use]
extern crate ss_view_tree;

pub mod dev;
pub mod trees;

use wasm_bindgen::prelude::*;
use ss_web_utils::js::{self, console};

#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console::log("started");
    Ok(())
}

#[wasm_bindgen]
pub fn tick() {
	
}






