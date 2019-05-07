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


///////////////////////////////////////////////////////////////////////////////
// SPEC - EXTERNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct InitArgs<Model> {
    pub saved_model: Option<Model>
}

pub struct Init<Model, Msg> {
    pub model: Model,
    pub subs: Rc<Fn(Rc<Any>)->Option<Msg>>,
}

#[derive(Clone)]
pub struct Cmd {
    pub update_view: Rc<Cell<bool>>,
    pub queued_commands: Rc<RefCell<VecDeque<CmdRequest>>>,
}

pub type Html<Msg> = HtmlBuild<Msg>;

pub trait Spec
where
    Self: Clone + 'static
{
    type Msg: Debug + Clone + 'static;
    type Model: Debug + Clone + Serialize + DeserializeOwned;
    
    fn init(&self, loaded: InitArgs<Self::Model>) -> Init<Self::Model, Self::Msg>;
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd);
    fn view(&self, model: &Self::Model) -> Html<Self::Msg>;
}


///////////////////////////////////////////////////////////////////////////////
// SPEC - INTERNAL
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone, Debug)]
pub enum CmdRequest {
    Save,
    Navigate(String),
    Broadcast(Rc<Any>),
}



///////////////////////////////////////////////////////////////////////////////
// META - GLOBAL-REGISTRY
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_REGISTRY: GlobalRegistry = {
        GlobalRegistry {
            processes: RefCell::new(HashMap::new())
        }
    };
}

pub struct GlobalRegistry {
    processes: RefCell<HashMap<ProcessId, Rc<ProcessHandle>>>,
}

impl GlobalRegistry {
    pub fn add_process(&self, process: Rc<ProcessHandle>) {
        self.processes.borrow_mut().insert(process.process_id(), process);
    }
    pub fn remove_process(&self, process_id: &ProcessId) {
        self.processes.borrow_mut().remove(process_id);
    }
    pub fn broadcast(&self, value: Rc<Any>, sender_pid: Option<&str>) {
        for process in self.processes.borrow().values() {
            let is_sender = {
                match sender_pid {
                    None => false,
                    Some(pid) => pid == process.process_id().as_str(),
                }
            };
            if !is_sender {
                process.receive_broadcast(value.clone());
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// APPLICATION
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppBuilder {
    pub effects: Vec<Rc<Effect>>,
    pub process: Rc<ProcessHandle>,
}


#[derive(Clone)]
pub struct Application {
    pub js_tick_callback: Rc<RefCell<Option<Callback<()>>>>,
    pub root_process: Rc<ProcessHandle>,
    pub effects: Vec<Rc<Effect>>,
}


///////////////////////////////////////////////////////////////////////////////
// PROCESS
///////////////////////////////////////////////////////////////////////////////

type ProcessId = String;

pub trait Effect {
    fn init(&self);
}

pub trait ProcessHandle {
    fn process_id(&self) -> String;
    fn dom_ref(&self) -> &DomRef;
    fn receive_broadcast(&self, value: Rc<Any>);
    fn clear(&self);
    fn tick(&self);
}

#[derive(Clone)]
pub struct Process<S: Spec> {
    pub process_id: ProcessId,
    pub spec: S,
    pub model: RefCell<S::Model>,
    pub subscriber: Rc<Fn(Rc<Any>)->Option<S::Msg>>,
    pub offline_html: RefCell<HtmlBuild<S::Msg>>,
    pub online_html: LiveHtml<S::Msg>,
    pub queued_messages: RefCell<VecDeque<S::Msg>>,
    pub queued_commands: Rc<RefCell<VecDeque<CmdRequest>>>,
    pub queued_anything: RefCell<VecDeque<Rc<Any>>>,
    pub sub_processes: RefCell<Vec<Rc<ProcessHandle>>>,
}


