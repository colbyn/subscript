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
use crate::process::registry::*;


///////////////////////////////////////////////////////////////////////////////
// PROCESS
///////////////////////////////////////////////////////////////////////////////

pub type ProcessId = String;

pub trait ProcessHandle {
    fn spec_type_id(&self) -> TypeId;
    fn process_id(&self) -> String;
    fn dom_ref(&self) -> &DomRef;
    fn tick(&self, sub_enqueue: &Vec<Rc<Any>>);
    fn box_clone(&self) -> Box<ProcessHandle>;
}

impl Clone for Box<ProcessHandle>
{
    fn clone(&self) -> Box<ProcessHandle> {
        self.box_clone()
    }
}


#[derive(Clone)]
pub struct Process<S: Spec>(pub Rc<ProcessInstance<S>>);

pub struct ProcessInstance<S: Spec> {
    pub process_name: Option<String>,
    pub process_id: ProcessId,
    pub spec: S,
    pub offline_model: RefCell<S::Model>,
    pub online_model: RefCell<S::Model>,
    pub subs: Subscriptions<S::Msg>,
    pub offline_html: RefCell<HtmlBuild<S::Msg>>,
    pub online_html: LiveHtml<S::Msg>,
    pub queued_commands: Rc<RefCell<VecDeque<CmdRequest>>>,
}


impl<S: Spec> Process<S> {
    pub fn from_spec(spec: S) -> Self {
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
                process_name: None,
                process_id: format!("pid-{}", rand::random::<u16>()),
                spec: spec,
                offline_model: RefCell::new(model.clone()),
                online_model: RefCell::new(model),
                subs: subs,
                offline_html: RefCell::new(offline_html),
                online_html: online_html,
                queued_commands: Rc::new(RefCell::new(VecDeque::new())),
            })
        );
        GLOBAL_REGISTRY.with(|reg| {
            reg.add_process(process.clone());
        });
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
    fn tick(&self, global_events: &Vec<Rc<Any>>) {
        let messages = {
            // SETUP
            let mut xs: Vec<S::Msg> = Vec::new();
            // FIRST - HTML DOM EVENTS
            self.0.online_html.tick(&mut xs, global_events);
            // SECOND - SUBSCRIPTIONS
            self.0.subs.tick(&mut xs, global_events);
            // DONE
            xs
        };
        if !messages.is_empty() {
            // PROCESS EVENTS
            let ref cmd = Cmd {
                queued_commands: self.0.queued_commands.clone(),
            };
            for msg in messages {
                self.0.spec.update(&mut self.0.online_model.borrow_mut(), msg, cmd);
            }
            // PROCESS VIEW
            let unchanged = {
                *self.0.online_model.borrow() == *self.0.offline_model.borrow()
            };
            if !unchanged {
                self.0.offline_html.replace(
                    self.0.spec.view(&self.0.online_model.borrow())
                );
                self.0.online_html.sync(&self.0.offline_html.borrow());
                self.0.offline_model
                    .replace(self.0.online_model.borrow().clone());
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
                            save_model::<S>(&self.0.offline_model.borrow());
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




