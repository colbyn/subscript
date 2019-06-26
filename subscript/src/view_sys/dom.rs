use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::backend::browser;
use crate::reactive_sys::*;
use crate::view_sys::dsl::{View};
use crate::view_sys::shared::*;
use crate::program_sys::instances::*;
use crate::program_sys::spec::*;
use crate::program_sys::shell::SystemMessage;

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
    Linked(ViewVecObserver<Msg>),
    Toggle(Box<Toggle<Msg>>),
    Dynamic(Box<Dynamic<Msg>>),
}

#[derive(Debug)]
pub(crate) struct Toggle<Msg> {
    pub pred: Formula<bool>,
    pub template: Rc<View<Msg>>,
    pub dom: RefCell<Option<Dom<Msg>>>,
}

#[derive(Debug)]
pub struct Dynamic<Msg> {
    pub(crate) producer: DynamicProducer<Msg>,
    pub(crate) view: Option<Dom<Msg>>,
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
    pub(crate) fn tick(&mut self, system_messages: &Vec<SystemMessage>) {
        (self.0).0.tick(system_messages);
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
    pub(crate) fn event_type(&self) -> String {self.frontend_callback.event_type()}
    pub(crate) fn tick(&self, tick_env: &mut TickEnv<Msg>) {
        for event in self.backend_callback.drain() {
            let msg = self.frontend_callback.apply(event);
            tick_env.local_messages.push(msg);
        }
    }
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

