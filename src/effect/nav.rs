#[macro_use]
pub mod router;

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
use crate::tree::offline::data::*;
use crate::tree::offline::api::*;
use crate::tree::online::data::*;
use crate::process::data::*;


///////////////////////////////////////////////////////////////////////////////
// NAVIGATION API
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct UrlChange {
    pub new_url: String,
}

#[derive(Clone, Debug)]
pub struct Navigation {
    js_nav_callback: Callback<()>,
}

impl Navigation {
    pub fn new() -> Self {
        Navigation {
            js_nav_callback: {
                let cb = Callback::new(Rc::new({
                    move |event: JsValue| -> Option<()> {
                        console::log("Event: onpopstate");
                        let event: web_sys::Event = From::from(event);
                        event.prevent_default();
                        let payload = UrlChange {
                            new_url: Browser::new().window
                                .location()
                                .pathname()
                                .expect("pathname failed")
                        };
                        GLOBAL_REGISTRY.with(|reg| {
                            reg.add_event(Rc::new(payload));
                        });
                        None
                    }
                }));
                let window = Browser::new().window;
                let window: web_sys::EventTarget = From::from(window);
                window.add_event_listener_with_callback_and_bool(
                    "popstate",
                    &cb.js_function,
                    false
                ).expect("addEventListener(...) for popstate failed.");
                cb
            },
        }
    }
}

impl Effect for Navigation {
    fn init(&self) {
        let payload = UrlChange {
            new_url: Browser::new().window
                .location()
                .pathname()
                .expect("pathname failed")
        };
        GLOBAL_REGISTRY.with(|reg| {
            reg.add_event(Rc::new(payload));
        });
    }
}


