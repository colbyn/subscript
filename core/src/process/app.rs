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
use wasm_bindgen::convert::*;

use crate::browser::*;
use crate::tree::offline::data::*;
use crate::tree::online::data::*;

use crate::process::basics::*;
use crate::process::offline::*;
use crate::process::online::*;
use crate::process::registry::*;


///////////////////////////////////////////////////////////////////////////////
// APPLICATION
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Application {
    pub js_tick_callback: Rc<RefCell<Option<Callback<()>>>>,
    pub root_process: Rc<ProcessHandle>,
}

impl Application {
    pub fn from_spec<S: Spec>(spec: S) -> Self {
        Application {
            js_tick_callback: Rc::new(RefCell::new(None)),
            root_process: Rc::new(Process::from_spec(spec)),
        }
    }
    
    pub fn tick(&self) {
        let ref global_events: Vec<Rc<Any>> = GLOBAL_REGISTRY.with(|reg| {
            reg.drain_events()
        });
        self.root_process.tick(global_events);
        GLOBAL_CSS.with(|css| {
            css.tick();
        });
    }
    pub fn start(self) {
        let browser = Browser::new();
        let handler: Rc<Fn(JsValue)->Option<()> > = Rc::new({
            let this = self.clone();
            move |_| {
                this.tick();
                Browser::new().window.request_animation_frame(
                    &this.js_tick_callback
                        .borrow()
                        .as_ref()
                        .expect("failed to tick")
                        .js_function
                );
                None
            }
        });
        let handler: Callback<()> = Callback::new(handler.clone());
        self.js_tick_callback.replace(Some(handler.clone()));
        browser.window.request_animation_frame(
            &handler.js_function
        );
        browser.body.append_child(self.root_process.dom_ref());
        std::mem::forget(self);
    }
}


