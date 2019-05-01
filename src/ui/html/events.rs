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
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;

use crate::browser::{self, Browser, Callback, console, DomRef};


#[derive(Clone, Debug)]
pub struct Events<Msg> {
    event_listeners: Rc<RefCell<HashMap<String, EventHandler<Msg>>>>,
}

impl<Msg: Clone + Debug + 'static> Events<Msg> {
    ///////////////////////////////////////////////////////////////////////////
    // EXTERNAL
    ///////////////////////////////////////////////////////////////////////////
    pub fn new() -> Self {
        Events {
            event_listeners: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    pub fn tick(&self) -> Vec<Msg> {
        let mut results: Vec<Msg> = Vec::new();
        for (key, handler) in self.event_listeners.borrow().iter() {
            results.append(&mut handler.callback.drain());
        }
        results
    }
    pub fn insert_event_handler(&self, event_name: String, fun: Rc<Fn(JsValue)->Msg>) {
        let handler =  EventHandler {
            callback: Callback::new({
                let fun = fun.clone();
                Rc::new(move |value| {Some(fun(value))})
            }),
            is_live: Cell::new(false),
        };
        self.event_listeners.borrow_mut().insert(event_name, handler);
    }
    pub fn init(&self, live: &DomRef) {
        for (key, handler) in self.event_listeners.borrow().iter() {
            if !handler.is_live.get() {
                live.add_event_listener(key, &handler.callback);
                handler.is_live.set(true);
            }
        }
    }
    pub fn clear(&self, live: &DomRef) {
        for (name, handler) in self.event_listeners.borrow().iter() {
            live.remove_event_listener(
                name.as_str(),
                &handler.callback,
            );
        }
    }
    pub fn sync(&self, other: &Events<Msg>, live: &DomRef) {
        // SETUP
        let unchanged: bool = {
            let ks1: Vec<String> = self.event_listeners
                .borrow()
                .keys()
                .map(|x| x.clone()).collect::<Vec<String>>();
            let ks2: Vec<String> = other.event_listeners
                .borrow()
                .keys()
                .map(|x| x.clone()).collect::<Vec<String>>();
            ks1 == ks2
        };
        if !unchanged {
            // REMOVE
            self.clear(live);
            self.event_listeners.replace(
                other.event_listeners.borrow().clone()
            );
            // ATTATCH
            self.init(live);
        }
    }
}

impl<Msg: Hash> Hash for Events<Msg> {
    fn hash<H: Hasher>(&self, state: &mut H) {}
}




///////////////////////////////////////////////////////////////////////////////
// EVENT-HANDLER
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone, Debug)]
pub struct EventHandler<Msg> {
    pub callback: Callback<Msg>,
    pub is_live: Cell<bool>,
}




