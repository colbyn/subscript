use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::backend::browser;
use crate::model_sys::reactive::{Signal, SignalSub, Status};
use crate::model_sys::incremental::{IVecSub};
use crate::view_sys::dsl::{View};
use crate::view_sys::shared::*;



///////////////////////////////////////////////////////////////////////////////
// DATA - LIVE DOM TREE
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(crate) enum Dom<Msg> {
    Component(LiveComponent),
    Text(Text),
    Element(Element<Msg>),
    Mixin(Mixin<Msg>),
    Control(Control<Msg>),
}

#[derive(Debug)]
pub(crate) struct Text {
    pub dom_ref: browser::Text,
    pub value: Value<String>,
}

#[derive(Debug)]
pub(crate) struct Element<Msg> {
    pub dom_ref: browser::Element,
    pub auto_listeners: Vec<browser::VoidCallback>,
    pub tag: String,
    pub styling: Styling,
    pub attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
    pub events: Vec<LiveEventHandler<Msg>>,
    pub children: Vec<Dom<Msg>>,
}

#[derive(Debug)]
pub(crate) struct Mixin<Msg> {
    pub styling: Styling,
    pub attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
    pub events: Vec<LiveEventHandler<Msg>>,
    pub children: Vec<Dom<Msg>>,
}

#[derive(Debug)]
pub(crate) enum Control<Msg> {
    Linked(IVecSub<Msg>),
    Toggle(Box<Toggle<Msg>>),
}

#[derive(Debug)]
pub(crate) struct Toggle<Msg> {
    pub pred: SignalSub<bool>,
    pub template: Rc<View<Msg>>,
    pub dom: RefCell<Option<Dom<Msg>>>,
}


///////////////////////////////////////////////////////////////////////////////
// DATA - SPECIAL - COMPONENTS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct LiveComponent(pub(crate) SubProcess);

impl LiveComponent {
    pub(crate) fn dom_ref(&self) -> browser::Element {
        (self.0).0.dom_ref()
    }
    pub(crate) fn tick(&mut self) {
        (self.0).0.tick()
    }
}


///////////////////////////////////////////////////////////////////////////////
// DATA - SPECIAL - PROPERTIES
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(crate) struct LiveEventHandler<Msg> {
    pub(crate) frontend_callback: EventHandler<Msg>,
    pub(crate) backend_callback: browser::QueueCallback,
}

impl<Msg> LiveEventHandler<Msg> {
    pub(crate) fn apply(&self, event: JsValue) -> Msg {self.frontend_callback.apply(event)}
    pub(crate) fn event_type(&self) -> String {self.frontend_callback.event_type()}
}

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS - DATA
///////////////////////////////////////////////////////////////////////////////

pub enum Link<New, Old> {
    New(New),
    Unchanged(Old),
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

impl<Msg> Dom<Msg> {
    pub(crate) fn unsafe_get_element(&self) -> &Element<Msg> {
        match self {
            Dom::Element(x) => x,
            _ => panic!()
        }
    }
}

