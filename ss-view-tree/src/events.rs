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

use ss_web_utils::js;
use ss_web_utils::dom;


///////////////////////////////////////////////////////////////////////////////
// DOM EVENT-HANDLERS
///////////////////////////////////////////////////////////////////////////////

pub fn on_click<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_mouse_down<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_mouse_up<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_mouse_enter<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_mouse_leave<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_mouse_over<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_mouse_out<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_input<Msg>(cb: fn(String)->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_check<Msg>(cb: fn(bool)->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_submit<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_blur<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}
pub fn on_focus<Msg>(cb: fn()->Msg) -> EventHandler<Msg> {
    unimplemented!()
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct EventHandlers<Msg>(pub(crate) BTreeMap<EventType, EventHandler<Msg>>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EventHandler<Msg>(pub(crate) IEventHandler<Msg>);

impl<Msg> EventHandler<Msg> {
    pub fn event_name(&self) -> EventType {
        match self {
            EventHandler(IEventHandler::OnClick(_)) => 
                EventType::OnClick,
            EventHandler(IEventHandler::OnMouseDown(_)) => 
                EventType::OnMouseDown,
            EventHandler(IEventHandler::OnMouseUp(_)) => 
                EventType::OnMouseUp,
            EventHandler(IEventHandler::OnMouseEnter(_)) => 
                EventType::OnMouseEnter,
            EventHandler(IEventHandler::OnMouseLeave(_)) => 
                EventType::OnMouseLeave,
            EventHandler(IEventHandler::OnMouseOver(_)) => 
                EventType::OnMouseOver,
            EventHandler(IEventHandler::OnMouseOut(_)) => 
                EventType::OnMouseOut,
            EventHandler(IEventHandler::OnInput(_)) => 
                EventType::OnInput,
            EventHandler(IEventHandler::OnCheck(_)) => 
                EventType::OnCheck,
            EventHandler(IEventHandler::OnSubmit(_)) => 
                EventType::OnSubmit,
            EventHandler(IEventHandler::OnBlur(_)) => 
                EventType::OnBlur,
            EventHandler(IEventHandler::OnFocus(_)) => 
                EventType::OnFocus,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// DOM events enum.
pub enum EventType {
    OnClick,
    OnMouseDown,
    OnMouseUp,
    OnMouseEnter,
    OnMouseLeave,
    OnMouseOver,
    OnMouseOut,
    OnInput,
    OnCheck,
    OnSubmit,
    OnBlur,
    OnFocus,
}

impl EventType {
    /// Gets the event name as a string.
    pub fn as_str(&self) -> &str {
        match self {
            EventType::OnClick => "click",
            EventType::OnMouseDown => "mousedown",
            EventType::OnMouseUp => "mouseup",
            EventType::OnMouseEnter => "mouseenter",
            EventType::OnMouseLeave => "mouseenter",
            EventType::OnMouseOver => "mouseover",
            EventType::OnMouseOut => "mouseout",
            EventType::OnInput => "change",
            EventType::OnCheck => "click",
            EventType::OnSubmit => "submit",
            EventType::OnBlur => "blur",
            EventType::OnFocus => "focus",
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum IEventHandler<Msg> {
    OnClick(OnClick<Msg>),
    OnMouseDown(OnMouseDown<Msg>),
    OnMouseUp(OnMouseUp<Msg>),
    OnMouseEnter(OnMouseEnter<Msg>),
    OnMouseLeave(OnMouseLeave<Msg>),
    OnMouseOver(OnMouseOver<Msg>),
    OnMouseOut(OnMouseOut<Msg>),
    OnInput(OnInput<Msg>),
    OnCheck(OnCheck<Msg>),
    OnSubmit(OnSubmit<Msg>),
    OnBlur(OnBlur<Msg>),
    OnFocus(OnFocus<Msg>),
}

// MOUSE
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnClick<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnMouseDown<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnMouseUp<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnMouseEnter<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnMouseLeave<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnMouseOver<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnMouseOut<Msg>(pub fn()->Msg);


// FORMS
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnInput<Msg>(pub fn(String)->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnCheck<Msg>(pub fn(bool)->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnSubmit<Msg>(pub fn()->Msg);


// FOCUS
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnBlur<Msg>(pub fn()->Msg);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OnFocus<Msg>(pub fn()->Msg);


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - NODE EVENT-HANDLERS
///////////////////////////////////////////////////////////////////////////////

impl<Msg: Ord> EventHandlers<Msg> {
    pub fn new() -> Self {
        EventHandlers(BTreeMap::new())
    }
    pub fn into_inner(self) -> Vec<EventHandler<Msg>> {
        let mut results = Vec::new();
        for (_, v) in self.0 {
            results.push(v);
        }
        results
    }
    pub fn add(&mut self, handler: EventHandler<Msg>) {
        let key = match &handler {
            EventHandler(IEventHandler::OnClick(_)) =>
                EventType::OnClick,
            EventHandler(IEventHandler::OnMouseDown(_)) =>
                EventType::OnMouseDown,
            EventHandler(IEventHandler::OnMouseUp(_)) =>
                EventType::OnMouseUp,
            EventHandler(IEventHandler::OnMouseEnter(_)) =>
                EventType::OnMouseEnter,
            EventHandler(IEventHandler::OnMouseLeave(_)) =>
                EventType::OnMouseLeave,
            EventHandler(IEventHandler::OnMouseOver(_)) =>
                EventType::OnMouseOver,
            EventHandler(IEventHandler::OnMouseOut(_)) =>
                EventType::OnMouseOut,
            EventHandler(IEventHandler::OnInput(_)) =>
                EventType::OnInput,
            EventHandler(IEventHandler::OnCheck(_)) =>
                EventType::OnCheck,
            EventHandler(IEventHandler::OnSubmit(_)) =>
                EventType::OnSubmit,
            EventHandler(IEventHandler::OnBlur(_)) =>
                EventType::OnBlur,
            EventHandler(IEventHandler::OnFocus(_)) =>
                EventType::OnFocus,
        };
        self.0.insert(key, handler);
    }
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL - INSTANCES
///////////////////////////////////////////////////////////////////////////////

// ALL
impl<Msg> js::Handler<Msg> for EventHandler<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0.handler(event)
    }
}
impl<Msg> js::Handler<Msg> for IEventHandler<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        match self {
            IEventHandler::OnClick(x) => x.handler(event),
            IEventHandler::OnMouseDown(x) => x.handler(event),
            IEventHandler::OnMouseUp(x) => x.handler(event),
            IEventHandler::OnMouseEnter(x) => x.handler(event),
            IEventHandler::OnMouseLeave(x) => x.handler(event),
            IEventHandler::OnMouseOver(x) => x.handler(event),
            IEventHandler::OnMouseOut(x) => x.handler(event),
            IEventHandler::OnInput(x) => x.handler(event),
            IEventHandler::OnCheck(x) => x.handler(event),
            IEventHandler::OnSubmit(x) => x.handler(event),
            IEventHandler::OnBlur(x) => x.handler(event),
            IEventHandler::OnFocus(x) => x.handler(event),
        }
    }
}

// MOUSE
impl<Msg> js::Handler<Msg> for OnClick<Msg> {
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

// FOCUS
impl<Msg> js::Handler<Msg> for OnBlur<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}
impl<Msg> js::Handler<Msg> for OnFocus<Msg> {
    fn handler(&self, event: JsValue) -> Msg {
        self.0()
    }
}




