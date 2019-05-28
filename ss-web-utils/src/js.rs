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
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;


///////////////////////////////////////////////////////////////////////////////
// CONSOLE
///////////////////////////////////////////////////////////////////////////////
pub mod console {
    use super::*;
    
    pub fn log(value: impl Loggable) {
        match value.to_js_value() {
            Either::Left(x) => {
                web_sys::console::log_1(&x);
            }
            Either::Right(x) => {
                web_sys::console::log_1(x);
            }
        }
    }
    pub fn warn(value: impl Loggable) {
        match value.to_js_value() {
            Either::Left(x) => {
                web_sys::console::warn_1(&x);
            }
            Either::Right(x) => {
                web_sys::console::warn_1(x);
            }
        }
    }
    
    pub trait Loggable {
        fn to_js_value(&self) -> Either<JsValue, &JsValue>;
    }
    impl Loggable for &str {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Left(JsValue::from_str(self))
        }
    }
    impl Loggable for String {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Left(JsValue::from_str(self.as_str()))
        }
    }
    impl Loggable for JsValue {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Right(&self)
        }
    }
    impl Loggable for &JsValue {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Right(self)
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// JAVASCRIPT VOID CALLBACK
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone)]
pub struct VoidCallback {
    pub i_bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
    pub i_js_function: Rc<js_sys::Function>,
}

impl VoidCallback {
    pub fn new(cb: impl Fn(JsValue) + 'static) -> Self {
        use wasm_bindgen::JsCast;
        let function_wrapper: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            move |value: JsValue| {
                cb(value)
            }
        }));
        let js_function: &js_sys::Function = function_wrapper.as_ref().unchecked_ref();
        let js_function: js_sys::Function = js_function.clone();
        let void_callback = VoidCallback {
            i_bindgen_closure: Rc::new(function_wrapper),
            i_js_function: Rc::new(js_function),
        };
        void_callback
    }
}

impl Debug for VoidCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "VoidCallback")
    }
}
impl PartialEq for VoidCallback {
    fn eq(&self, other: &VoidCallback) -> bool {true}
}
impl crate::dom::Callback for VoidCallback {
    fn as_js_function(&self) -> &js_sys::Function {
        self.i_js_function.as_ref()
    }
}

///////////////////////////////////////////////////////////////////////////////
// JAVASCRIPT QUEUE CALLBACK
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone)]
pub struct QueueCallback {
    pub i_settings: Rc<RefCell<Option<DomCallbackSettings>>>,
    pub i_bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
    pub i_js_function: Rc<js_sys::Function>,
    pub i_events: Rc<RefCell<VecDeque<JsValue>>>,
}

impl QueueCallback {
    pub fn new() -> Self {
        use wasm_bindgen::JsCast;
        let events_queue = Rc::new(RefCell::new(VecDeque::new()));
        let i_settings = Rc::new(RefCell::new(None));
        let function_wrapper: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let events_queue = events_queue.clone();
            let i_settings = i_settings.clone();
            move |value: JsValue| {
                events_queue.borrow_mut().push_back(value);
            }
        }));
        let js_function: &js_sys::Function = function_wrapper.as_ref().unchecked_ref();
        let js_function: js_sys::Function = js_function.clone();
        let queue_callback = QueueCallback {
            i_settings,
            i_bindgen_closure: Rc::new(function_wrapper),
            i_js_function: Rc::new(js_function),
            i_events: events_queue,
        };
        queue_callback
    }
    pub fn drain(&self) -> Vec<JsValue> {
        let xs: Vec<JsValue> = self.i_events.borrow_mut().drain(..).collect();
        xs
    }
}

impl Debug for QueueCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "QueueCallback")
    }
}
impl PartialEq for QueueCallback {
    fn eq(&self, other: &QueueCallback) -> bool {true}
}

impl crate::dom::Callback for QueueCallback {
    fn as_js_function(&self) -> &js_sys::Function {
        self.i_js_function.as_ref()
    }
}





///////////////////////////////////////////////////////////////////////////////
// JAVASCRIPT DOM EVENT-HANDLER
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct DomCallbackSettings {
    pub stop_propagation: bool,
    pub prevent_default: bool,
}

#[derive(Clone)]
pub struct DomEventHandler {
    pub i_settings: Rc<RefCell<DomCallbackSettings>>,
    pub i_bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
    pub i_js_function: Rc<js_sys::Function>,
    pub i_events: Rc<RefCell<VecDeque<JsValue>>>,
}

impl DomEventHandler {
    pub fn new(settings: DomCallbackSettings) -> Self {
        fn handler(settings: Rc<RefCell<DomCallbackSettings>>, value: &JsValue) {
            let settings: DomCallbackSettings = settings.borrow().clone();
            let event: web_sys::Event = From::from(value.clone());
            if settings.prevent_default {
                event.prevent_default();
            }
            if settings.stop_propagation {
                event.stop_propagation();
            }
        }
        let events_queue = Rc::new(RefCell::new(VecDeque::new()));
        let i_settings = Rc::new(RefCell::new(settings));
        let function_wrapper: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let events_queue = events_queue.clone();
            let i_settings = i_settings.clone();
            move |value: JsValue| {
                // handler(i_settings.clone(), &value);
                events_queue.borrow_mut().push_back(value);
            }
        }));
        let js_function: &js_sys::Function = function_wrapper.as_ref().unchecked_ref();
        let js_function: js_sys::Function = js_function.clone();
        let queue_callback = DomEventHandler {
            i_settings,
            i_bindgen_closure: Rc::new(function_wrapper),
            i_js_function: Rc::new(js_function),
            i_events: events_queue,
        };
        queue_callback
    }
    pub fn drain(&self) -> Vec<JsValue> {
        let xs: Vec<JsValue> = self.i_events.borrow_mut().drain(..).collect();
        xs
    }
}

impl Debug for DomEventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "DomEventHandler")
    }
}
impl PartialEq for DomEventHandler {
    fn eq(&self, other: &DomEventHandler) -> bool {true}
}

impl crate::dom::Callback for DomEventHandler {
    fn as_js_function(&self) -> &js_sys::Function {
        self.i_js_function.as_ref()
    }
}





