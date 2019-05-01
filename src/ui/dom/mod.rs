pub mod style_mount;

use std::fmt;
use std::fmt::Debug;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use either::Either;
use serde::{self, Serialize, Deserialize};
use web_sys::console;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;





///////////////////////////////////////////////////////////////////////////////
// INTERNAL UTILS
///////////////////////////////////////////////////////////////////////////////
pub fn get_window() -> web_sys::Window {
    let window: web_sys::Window = web_sys::window()
        .expect("window not available");
    window
}
pub fn get_document() -> web_sys::Document {
    let window: web_sys::Window = web_sys::window()
        .expect("window not available");
    let document = window
        .document()
        .expect("document not available");
    document
}
pub fn get_body() -> web_sys::Element {
    let window: web_sys::Window = web_sys::window()
        .expect("window not available");
    let document = window
        .document()
        .expect("document not available");
    let body: web_sys::Element = std::convert::From::from(
        document.body().expect("document.body not available")
    );
    body
}
pub fn new_element(tag: &String) -> web_sys::Element {
    let document = get_document();
    let result = document.create_element(tag.as_str())
        .expect("failed to create element");
    result
}
pub fn new_text(value: &String) -> web_sys::Text {
    let document = get_document();
    let result = document.create_text_node(value.as_str());
    result
}


