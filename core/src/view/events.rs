use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::Any;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use either::Either;
use wasm_bindgen::JsValue;

use crate::sys::js;
use crate::sys::dom;

// MOUSE
pub struct OnClick<Msg>(pub fn()->Msg);
pub struct OnDoubleClick<Msg>(pub fn()->Msg);
pub struct OnMouseDown<Msg>(pub fn()->Msg);
pub struct OnMouseUp<Msg>(pub fn()->Msg);
pub struct OnMouseEnter<Msg>(pub fn()->Msg);
pub struct OnMouseLeave<Msg>(pub fn()->Msg);
pub struct OnMouseOver<Msg>(pub fn()->Msg);
pub struct OnMouseOut<Msg>(pub fn()->Msg);

// FORMS
pub struct OnInput<Msg>(pub fn(String)->Msg);
pub struct OnCheck<Msg>(pub fn(bool)->Msg);
pub struct OnSubmit<Msg>(pub fn()->Msg);

// FOCUS
pub struct OnBlur<Msg>(pub fn()->Msg);
pub struct OnFocus<Msg>(pub fn()->Msg);


///////////////////////////////////////////////////////////////////////////////
// INTERNAL
///////////////////////////////////////////////////////////////////////////////

// MOUSE
impl<Msg> js::Handler<Msg> for OnClick<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnDoubleClick<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseDown<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseUp<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseEnter<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseLeave<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseOver<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnMouseOut<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}


// FORMS
impl<Msg> js::Handler<Msg> for OnInput<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0(dom::event::get_oninput_value(&event))
    }
}
impl<Msg> js::Handler<Msg> for OnCheck<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        let event: web_sys::Event = From::from(event.clone());
        let target: web_sys::EventTarget = event
            .target()
            .expect("target failed");
        let target: JsValue = From::from(target);
        let target: web_sys::HtmlInputElement = From::from(target);
        let value: bool = target.checked();
        self.0(value)
    }
}
impl<Msg> js::Handler<Msg> for OnSubmit<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}




