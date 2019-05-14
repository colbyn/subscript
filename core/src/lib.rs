#![allow(dead_code, unused)]
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate cached;
use wasm_bindgen::prelude::*;

pub mod browser;
#[macro_use]
pub mod tree;
#[macro_use]
pub mod process;
#[macro_use]
pub mod effect;
pub mod extras;

pub mod toolkit;

pub mod sys;
#[macro_use]
pub mod view;

pub mod dev;


#[wasm_bindgen]
pub fn main() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    // dev::client::main();
    browser::console::log(format!(
        "{:#?}",
        std::i8::MAX
    ));
    Ok(())
}




