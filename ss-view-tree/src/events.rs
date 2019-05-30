use std::marker::Sized;
use std::marker::PhantomData;
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
use crate::{Mixin, Viewable};


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL VIEW EVENT-HANDLERS
///////////////////////////////////////////////////////////////////////////////

pub fn on_click<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnClick(OnClick(Rc::new(cb)))
}
pub fn on_mouse_down<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnMouseDown(OnMouseDown(Rc::new(cb)))
}
pub fn on_mouse_up<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnMouseUp(OnMouseUp(Rc::new(cb)))
}
pub fn on_mouse_enter<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnMouseEnter(OnMouseEnter(Rc::new(cb)))
}
pub fn on_mouse_leave<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnMouseLeave(OnMouseLeave(Rc::new(cb)))
}
pub fn on_mouse_over<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnMouseOver(OnMouseOver(Rc::new(cb)))
}
pub fn on_mouse_out<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnMouseOut(OnMouseOut(Rc::new(cb)))
}
pub fn on_input<Msg>(cb: impl Fn(String) -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnInput(OnInput(Rc::new(cb)))
}
pub fn on_check<Msg>(cb: impl Fn(bool) -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnCheck(OnCheck(Rc::new(cb)))
}
pub fn on_submit<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnSubmit(OnSubmit(Rc::new(cb)))
}
pub fn on_blur<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnBlur(OnBlur(Rc::new(cb)))
}
pub fn on_focus<Msg>(cb: impl Fn() -> Msg + 'static) -> impl Viewable<Msg> {
    EventHandler::OnFocus(OnFocus(Rc::new(cb)))
}

///////////////////////////////////////////////////////////////////////////////
// INTERNAL MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

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
            EventType::OnMouseLeave => "mouseleave",
            EventType::OnMouseOver => "mouseover",
            EventType::OnMouseOut => "mouseout",
            EventType::OnInput => "input",
            EventType::OnCheck => "click",
            EventType::OnSubmit => "submit",
            EventType::OnBlur => "blur",
            EventType::OnFocus => "focus",
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL REPRESENTATION
///////////////////////////////////////////////////////////////////////////////

pub trait EventHandlerObject<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg;
    fn event_name(&self) -> EventType;
}

#[derive(Clone)]
pub enum EventHandler<Msg> {
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


impl<Msg> Viewable<Msg> for EventHandler<Msg> {
    fn mixin<'a>(self, mixin: Mixin<'a, Msg>) {
        mixin.events.insert(self.event_name(), self);
    }
}
impl<Msg> PartialEq for EventHandler<Msg> {
    fn eq(&self, other: &EventHandler<Msg>) -> bool {
        self.event_name() == other.event_name()
    }
}
impl<Msg> Debug for EventHandler<Msg> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f, "EventHandler")}
}



// MOUSE
#[derive(Clone)]
pub struct OnClick<Msg>(Rc<Fn() -> Msg>); // click
#[derive(Clone)]
pub struct OnMouseDown<Msg>(Rc<Fn()->Msg>); // mousedown
#[derive(Clone)]
pub struct OnMouseUp<Msg>(Rc<Fn()->Msg>); // mouseup
#[derive(Clone)]
pub struct OnMouseEnter<Msg>(Rc<Fn()->Msg>); // mouseenter
#[derive(Clone)]
pub struct OnMouseLeave<Msg>(Rc<Fn()->Msg>); // mouseenter
#[derive(Clone)]
pub struct OnMouseOver<Msg>(Rc<Fn()->Msg>); // mouseover
#[derive(Clone)]
pub struct OnMouseOut<Msg>(Rc<Fn()->Msg>); // mouseout


// FORMS
#[derive(Clone)]
pub struct OnInput<Msg>(Rc<Fn(String)->Msg>); // change
#[derive(Clone)]
pub struct OnCheck<Msg>(Rc<Fn(bool)->Msg>); // click
#[derive(Clone)]
pub struct OnSubmit<Msg>(Rc<Fn()->Msg>); // submit


// FOCUS
#[derive(Clone)]
pub struct OnBlur<Msg>(Rc<Fn()->Msg>); // blur
#[derive(Clone)]
pub struct OnFocus<Msg>(Rc<Fn()->Msg>); // focus



///////////////////////////////////////////////////////////////////////////////
// INTERNAL - APPLY-HANDLER
///////////////////////////////////////////////////////////////////////////////

impl<Msg> EventHandlerObject<Msg> for EventHandler<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        match self {
            EventHandler::OnClick(x) => {x.run_handler(event)}
            EventHandler::OnMouseDown(x) => {x.run_handler(event)}
            EventHandler::OnMouseUp(x) => {x.run_handler(event)}
            EventHandler::OnMouseEnter(x) => {x.run_handler(event)}
            EventHandler::OnMouseLeave(x) => {x.run_handler(event)}
            EventHandler::OnMouseOver(x) => {x.run_handler(event)}
            EventHandler::OnMouseOut(x) => {x.run_handler(event)}
            EventHandler::OnInput(x) => {x.run_handler(event)}
            EventHandler::OnCheck(x) => {x.run_handler(event)}
            EventHandler::OnSubmit(x) => {x.run_handler(event)}
            EventHandler::OnBlur(x) => {x.run_handler(event)}
            EventHandler::OnFocus(x) => {x.run_handler(event)}
        }
    }
    fn event_name(&self) -> EventType {
        match self {
            EventHandler::OnClick(_) => EventType::OnClick,
            EventHandler::OnMouseDown(_) => EventType::OnMouseDown,
            EventHandler::OnMouseUp(_) => EventType::OnMouseUp,
            EventHandler::OnMouseEnter(_) => EventType::OnMouseEnter,
            EventHandler::OnMouseLeave(_) => EventType::OnMouseLeave,
            EventHandler::OnMouseOver(_) => EventType::OnMouseOver,
            EventHandler::OnMouseOut(_) => EventType::OnMouseOut,
            EventHandler::OnInput(_) => EventType::OnInput,
            EventHandler::OnCheck(_) => EventType::OnCheck,
            EventHandler::OnSubmit(_) => EventType::OnSubmit,
            EventHandler::OnBlur(_) => EventType::OnBlur,
            EventHandler::OnFocus(_) => EventType::OnFocus,
        }
    }
}

// MOUSE
impl<Msg> EventHandlerObject<Msg> for OnClick<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnClick
    }
}
impl<Msg> EventHandlerObject<Msg> for OnMouseDown<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseDown
    }
}
impl<Msg> EventHandlerObject<Msg> for OnMouseUp<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseUp
    }
}
impl<Msg> EventHandlerObject<Msg> for OnMouseEnter<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseEnter
    }
}
impl<Msg> EventHandlerObject<Msg> for OnMouseLeave<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseLeave
    }
}
impl<Msg> EventHandlerObject<Msg> for OnMouseOver<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseOver
    }
}
impl<Msg> EventHandlerObject<Msg> for OnMouseOut<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseOut
    }
}


// FORMS
impl<Msg> EventHandlerObject<Msg> for OnInput<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0(dom::event::get_oninput_value(&event))
    }
    fn event_name(&self) -> EventType {
        EventType::OnMouseLeave
    }
}
impl<Msg> EventHandlerObject<Msg> for OnCheck<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        let event: web_sys::Event = From::from(event.clone());
        let target: web_sys::EventTarget = event
            .target()
            .expect("target failed");
        let target: JsValue = From::from(target);
        let target: web_sys::HtmlInputElement = From::from(target);
        let value: bool = target.checked();
        self.0(value)
    }
    fn event_name(&self) -> EventType {
        EventType::OnCheck
    }
}
impl<Msg> EventHandlerObject<Msg> for OnSubmit<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
        EventType::OnSubmit
    }
}

// FOCUS
impl<Msg> EventHandlerObject<Msg> for OnBlur<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
            EventType::OnBlur
        }
}
impl<Msg> EventHandlerObject<Msg> for OnFocus<Msg> {
    fn run_handler(&self, event: JsValue) -> Msg {
        self.0()
    }
    fn event_name(&self) -> EventType {
            EventType::OnFocus
        }
}




