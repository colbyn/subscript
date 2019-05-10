#[macro_use]
pub mod parser;

use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::any::*;
use std::marker::Sized;
use either::Either;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::browser::*;
use crate::process::basics::*;

pub use parser::UrlParser;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub struct Url(pub String);

pub fn mk_reactive() -> Reactive<Url> {
    fn get_current_value() -> Url {
        let value = Browser::new().window
            .location()
            .pathname()
            .expect("pathname failed");
        Url(value)
    }
    let listener = Callback::new(Rc::new(|event| {
        let event: web_sys::Event = From::from(event);
        event.prevent_default();
        Some(get_current_value())
    }));
    Reactive(ReactiveValue::JsHandler{
        value: Rc::new(RefCell::new(get_current_value())),
        listener: listener,
    })
}




