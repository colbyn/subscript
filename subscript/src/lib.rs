#![allow(dead_code, unused)]

#[macro_use]
pub mod backend;
#[macro_use]
pub mod view_sys;
#[macro_use]
pub mod program_sys;
// pub mod dev;
pub mod reactive_sys;
pub mod prelude;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn main() -> Result<(), wasm_bindgen::JsValue> {
//     console_error_panic_hook::set_once();
//     console!("started");
//     // dev::cms_app::client::setup();
//     // dev::todo_app::setup();
//     Ok(())
// }

// #[wasm_bindgen]
// pub fn tick() {
//     // dev::cms_app::client::tick();
//     // dev::todo_app::tick();
// }






