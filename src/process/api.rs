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
use crate::process::data::*;

///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Cmd {
    pub fn update_view(&self) {
        self.update_view.set(true);
    }
    pub fn navigate(&self, route: &str) {
        self.queued_commands.borrow_mut().push_back(
            CmdRequest::Navigate(String::from(route))
        );
    }
    pub fn save(&self) {
        self.queued_commands.borrow_mut().push_back(
            CmdRequest::Save
        );
    }
    pub fn broadcast(&self, value: impl Any) {
        self.queued_commands.borrow_mut().push_back(
            CmdRequest::Broadcast(Rc::new(value))
        );
    }
}


///////////////////////////////////////////////////////////////////////////////
// APPLICATION
///////////////////////////////////////////////////////////////////////////////

impl AppBuilder {
    pub fn from_spec<S: Spec>(spec: S) -> Self {
        AppBuilder {
            effects: Vec::new(),
            process: Rc::new(Process::from_spec(spec)),
        }
    }
    pub fn with_effect(mut self, effect: impl Effect + 'static) -> Self {
        self.effects.push(Rc::new(effect));
        self
    }
    pub fn build(self) -> Application {
        GLOBAL_REGISTRY.with(|reg| {
            reg.add_process(self.process.clone());
        });
        Application {
            js_tick_callback: Rc::new(RefCell::new(None)),
            root_process: self.process,
            effects: self.effects
        }
    }
}

impl Application {
    pub fn tick(&self) {
        self.root_process.tick();
        GLOBAL_CSS.with(|css| {
            css.tick();
        });
    }
    pub fn start(self) {
        for effect in self.effects.iter() {
            effect.init();
        }
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


///////////////////////////////////////////////////////////////////////////////
// PROCESS
///////////////////////////////////////////////////////////////////////////////

impl<S: Spec> Process<S> {
    pub fn from_spec(spec: S) -> Self {
        let Init{model, subs} = spec.init(InitArgs {
            saved_model: load_saved_model::<S>(),
        });
        let offline_html = spec.view(&model);
        let online_html = {
            let root = LiveHtml::from_builder(offline_html.clone());
            match root {
                LiveHtml::Component(_) => {
                    let msg = "The root view type of a process is a component,
the root view type of a process doesn’t change. Recommend changing such be a
child of an html node (e.g. where the root-most value is a div node).";
                    console::warn(msg);
                }
                _ => {}
            }
            root
        };
        Process {
            process_id: format!("pid-{}", rand::random::<u16>()),
            spec: spec,
            model: RefCell::new(model),
            subscriber: subs,
            offline_html: RefCell::new(offline_html),
            online_html: online_html,
            queued_commands: Rc::new(RefCell::new(VecDeque::new())),
            queued_anything: RefCell::new(VecDeque::new()),
            queued_messages: RefCell::new(VecDeque::new()),
            sub_processes: RefCell::new(Vec::new()),
        }
    }
}

impl<S: Spec> ProcessHandle for Process<S> {
    fn process_id(&self) -> String {
        self.process_id.clone()
    }
    fn dom_ref(&self) -> &DomRef {
        self.online_html.dom_ref()
    }
    fn receive_broadcast(&self, value: Rc<Any>) {
        self.queued_anything
            .borrow_mut()
            .push_back(value);
    }
    fn tick(&self) {
        let messages = {
            let mut xs: Vec<S::Msg> = Vec::new();
            self.online_html.tick(&mut xs);
            for something in self.queued_anything.borrow_mut().drain(..) {
                match self.subscriber.as_ref()(something.clone()) {
                    None => (),
                    Some(msg) => xs.push(msg),
                }
            }
            xs
        };
        if !messages.is_empty() {
            // PROCESS EVENTS
            let ref cmd = Cmd {
                update_view: Rc::new(Cell::new(false)),
                queued_commands: self.queued_commands.clone(),
            };
            for msg in messages {
                self.spec.update(&mut self.model.borrow_mut(), msg, cmd);
            }
            // PROCESS VIEW
            if cmd.update_view.get() {
                self.offline_html.replace(
                    self.spec.view(&self.model.borrow())
                );
                self.online_html.sync(&self.offline_html.borrow());
            }
            // PROCESS COMMANDS
            let queued_commands = self.queued_commands
                .borrow_mut()
                .drain(..)
                .collect::<Vec<CmdRequest>>();
            if !queued_commands.is_empty() {
                let browser = Browser::new();
                let local_storage = LocalStorage::new();
                for cmd in queued_commands {
                    match cmd {
                        CmdRequest::Navigate(route) => {
                            browser.window
                                .history()
                                .expect("history failed")
                                .push_state_with_url(
                                    &JsValue::null(),
                                    "",
                                    Some(route.as_str())
                                )
                                .expect("pushState failed");
                        }
                        CmdRequest::Save => {
                            save_model::<S>(&self.model.borrow());
                        }
                        CmdRequest::Broadcast(value) => {
                            GLOBAL_REGISTRY.with(|reg| {
                                reg.broadcast(value, Some(self.process_id.as_str()));
                            });
                        }
                    }
                }
            }
        }
    }
    fn clear(&self) {
        GLOBAL_REGISTRY.with(|reg| {
            reg.remove_process(&self.process_id);
        });
        self.online_html.clear();
    }
    fn init(&self) {
        // PROCESS VIEW
        self.offline_html.replace(
            self.spec.view(&self.model.borrow())
        );
        self.online_html.init();
        self.online_html.sync(&self.offline_html.borrow());
    }
}



pub fn save_model<S: Spec>(model: &S::Model) {
    let local_storage = LocalStorage::new();
    let spec_type_id = format!("{:?}", std::any::TypeId::of::<S>());
    local_storage.set::<S::Model>(spec_type_id.as_ref(), model);
}

pub fn load_saved_model<S: Spec>() -> Option<S::Model> {
    let local_storage = LocalStorage::new();
    let spec_type_id = format!("{:?}", std::any::TypeId::of::<S>());
    local_storage.get::<S::Model>(spec_type_id.as_ref())
}



#[macro_export]
macro_rules! subscriptions {
    ($(on($value:ident: $ty:ty) -> $msg_ty:ty {$body:expr})*) => {
        Rc::new({
            move |something: Rc<Any>| {
                let mut return_value = None;
                $(
                    {
                        if let Some($value) = something.downcast_ref::<$ty>() {
                            let $value = $value.clone();
                            if return_value.is_none() {
                                return_value = Some({$body});
                            }
                        }
                    }
                )*
                return_value
            }
        })
    };
}




