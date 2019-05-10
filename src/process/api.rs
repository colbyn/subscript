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
// SPEC - SUBSCRIPTIONS
///////////////////////////////////////////////////////////////////////////////


impl<Msg> Subscriptions<Msg> {
    pub fn tick(&self, messages: &mut Vec<Msg>, global_events: &Vec<Rc<Any>>) {
        for something in global_events {
            for fun in self.global_events.iter() {
                match fun(something.clone()) {
                    None => (),
                    Some(msg) => messages.push(msg),
                }
            }
        }
        for reactive in self.reactive_observers.iter() {
            match reactive() {
                None => (),
                Some(msg) => messages.push(msg),
            }
        }
    }
}

impl<Msg> Default for Subscriptions<Msg> {
    fn default() -> Self {
        Subscriptions {
            global_events: Rc::new(Vec::new()),
            reactive_observers: Rc::new(Vec::new()),
        }
    }
}


/// Helper data type for the 'subscriptions' macro.
pub struct SubBuilder<Msg> {
    pub global_events: Vec<Box<Fn(Rc<Any>)->Option<Msg>>>,
    pub reactive_observers: Vec<Box<Fn()->Option<Msg>>>,
}

#[macro_export]
macro_rules! subscription_body {
    ($body:expr) => {{$body}};
}

#[macro_export]
macro_rules! subscriptions_impl {
    ($sb:expr;) => {};
    ($sb:expr; bind($self:ident . $react:ident -> $new:ident) -> $msg:ty {$($xs:tt)*} $($rest:tt)*) => {{
        $sb.reactive_observers.push(Box::new({
            let this = $self.clone();
            move || {
                let this = this.clone();
                let reactive = this.$react;
                
                match reactive.0 {
                    ReactiveValue::JsHandler{value, listener} => {
                        if let Some(last_event) = listener.drain().last() {
                            value.replace(last_event.clone());
                            let $new = last_event.clone();
                            Some(subscription_body!({$($xs)*}))
                        } else {
                            None
                        }
                    }
                    ReactiveValue::Mutable{v1, v2} => {
                        let unchanged = {
                            *v1.borrow() == *v2.borrow()
                        };
                        console::log(format!("unchanged: {:#?}", unchanged));
                        if unchanged {
                            None
                        } else {
                            let $new = v2.borrow().clone();
                            v1.replace($new.clone());
                            Some(subscription_body!({$($xs)*}))
                        }
                    }
                }
            }
        }));
        subscriptions_impl!($sb; $($rest)*);
    }};
    
    ($sb:expr; on($value:ident: $ty:ty) -> $msg_ty:ty {$($x:tt)*} $($rest:tt)*) => {{
        $sb.global_events.push(
            Box::new({move |something: Rc<Any>| {
                let mut return_value = None;
                if let Some($value) = something.downcast_ref::<$ty>() {
                    let $value = $value.clone();
                    if return_value.is_none() {
                        return_value = Some(subscription_body!({$($x)*}));
                    }
                }
                return_value
            }})
        );
        subscriptions_impl!($sb; $($rest)*);
    }};
    
    ($sb:expr; $self:ty {$($xs:tt)*} $($rest:tt)*) => {{
        self_reactive_observers!($sb; $self; $($xs)*);
        subscriptions_impl!($sb; $($rest)*);
    }};
}

#[macro_export]
macro_rules! subscriptions {
    () => {{
        use crate::process::api::*;
        use crate::process::data::*;
        
        Subscriptions {
            global_events: Rc::new(Vec::new()),
            reactive_observers: Rc::new(Vec::new()),
        }
    }};
    ($($xs:tt)*) => {{
        use crate::process::api::*;
        use crate::process::data::*;
        
        let mut sb = SubBuilder {
            global_events: Vec::new(),
            reactive_observers: Vec::new(),
        };
        subscriptions_impl!(sb; $($xs)*);
        Subscriptions {
            global_events: Rc::new(sb.global_events),
            reactive_observers: Rc::new(sb.reactive_observers),
        }
    }};
}


///////////////////////////////////////////////////////////////////////////////
// SPEC - SUBSCRIPTIONS - REACTIVE
///////////////////////////////////////////////////////////////////////////////

impl<T: PartialEq + Clone> Reactive<T> {
    pub fn from_value(value: T) -> Self {
        Reactive(ReactiveValue::Mutable {
            v1: Rc::new(RefCell::new(value.clone())),
            v2: Rc::new(RefCell::new(value)),
        })
    }
    pub fn unlock(&self, key: &InitKey) -> T {
        match &self.0 {
            ReactiveValue::JsHandler{value, ..} => value.borrow().clone(),
            ReactiveValue::Mutable{v2, ..} => v2.borrow().clone(),
        }
    }
    pub fn set(&self, new: T) {
        match &self.0 {
            ReactiveValue::Mutable{v2, ..} => {
                v2.replace(new);
            },
            _ => ()
        }
    }
}




///////////////////////////////////////////////////////////////////////////////
// GLOBAL REGISTRY
///////////////////////////////////////////////////////////////////////////////

impl GlobalRegistry {
    pub fn drain_events(&self) -> Vec<Rc<Any>> {
        self.events
            .borrow_mut()
            .drain(..)
            .collect::<Vec<Rc<Any>>>()
    }
    pub fn add_event(&self, event: Rc<Any>) {
        self.events.borrow_mut().push(event);
    }
}


///////////////////////////////////////////////////////////////////////////////
// APPLICATION
///////////////////////////////////////////////////////////////////////////////

impl AppBuilder {
    pub fn from_spec<S: Spec>(spec: S) -> Self {
        AppBuilder {
            process: Rc::new(Process::from_spec("Root Process", spec)),
        }
    }
    pub fn build(self) -> Application {
        Application {
            js_tick_callback: Rc::new(RefCell::new(None)),
            root_process: self.process,
        }
    }
}

impl Application {
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


///////////////////////////////////////////////////////////////////////////////
// PROCESS
///////////////////////////////////////////////////////////////////////////////

impl<S: Spec> Process<S> {
    pub fn from_spec(process_name: &str, spec: S) -> Self {
        let Init{model, subs} = {
            let key = InitKey(());
            let args = InitArgs {
                saved_model: load_saved_model::<S>(),
            };
            spec.init(args, &key)
        };
        let offline_html = spec.view(&model);
        let online_html = {
            let root = LiveHtml::from_builder(offline_html.clone());
            match root {
                LiveHtml::Component(_) => {
                    console::warn("The root view type of a process is a component.");
                }
                _ => {}
            }
            root
        };
        let process = Process(
            Rc::new(ProcessInstance {
                process_name: Some(String::from(process_name)),
                process_id: format!("pid-{}", rand::random::<u16>()),
                spec: spec,
                model: RefCell::new(model),
                subs: subs,
                offline_html: RefCell::new(offline_html),
                online_html: online_html,
                queued_commands: Rc::new(RefCell::new(VecDeque::new())),
            })
        );
        process
    }
}

impl<S: Spec> ProcessHandle for Process<S> {
    fn spec_type_id(&self) -> TypeId {
        TypeId::of::<S>()
    }
    fn process_id(&self) -> String {
        self.0.process_id.clone()
    }
    fn dom_ref(&self) -> &DomRef {
        self.0.online_html.dom_ref()
    }
    fn start(&self) {
        
    }
    fn tick(&self, global_events: &Vec<Rc<Any>>) {
        let messages = {
            // SETUP
            let mut xs: Vec<S::Msg> = Vec::new();
            // SUBSCRIPTIONS
            self.0.subs.tick(&mut xs, global_events);
            // HTML DOM EVENTS
            self.0.online_html.tick(&mut xs, global_events);
            // DONE
            xs
        };
        if !messages.is_empty() {
            // PROCESS EVENTS
            let ref cmd = Cmd {
                update_view: Rc::new(Cell::new(false)),
                queued_commands: self.0.queued_commands.clone(),
            };
            for msg in messages {
                self.0.spec.update(&mut self.0.model.borrow_mut(), msg, cmd);
            }
            // PROCESS VIEW
            if cmd.update_view.get() {
                self.0.offline_html.replace(
                    self.0.spec.view(&self.0.model.borrow())
                );
                self.0.online_html.sync(&self.0.offline_html.borrow());
            }
            // PROCESS COMMANDS
            let queued_commands = self.0.queued_commands
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
                            save_model::<S>(&self.0.model.borrow());
                        }
                        CmdRequest::Broadcast(value) => {
                            GLOBAL_REGISTRY.with(|reg| {
                                reg.add_event(value);
                            });
                        }
                    }
                }
            }
        }
    }
    fn box_clone(&self) -> Box<ProcessHandle> {
        Box::new(self.clone())
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





