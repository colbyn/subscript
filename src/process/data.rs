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
// INTERNAL GLOBAL REGISTRY
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_REGISTRY: GlobalRegistry = {
        GlobalRegistry {
            events: RefCell::new(Vec::new())
        }
    };
}

pub struct GlobalRegistry {
    pub events: RefCell<Vec<Rc<Any>>>,
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

pub type ProcessId = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Online,
    Offline
}

pub trait Effect {
    fn init(&self);
}

pub trait ProcessHandle {
    fn process_id(&self) -> String;
    fn dom_ref(&self) -> &DomRef;
    fn status(&self) -> Status;
    fn tick(&self, sub_enqueue: &Vec<Rc<Any>>);
    fn online(&self);
    fn offline(&self);
    fn box_clone(&self) -> Box<ProcessHandle>;
}

impl Clone for Box<ProcessHandle>
{
    fn clone(&self) -> Box<ProcessHandle> {
        self.box_clone()
    }
}



#[derive(Clone)]
pub struct Process<S: Spec> {
    pub rc: Rc<ProcessInstance<S>>,
}

pub struct ProcessInstance<S: Spec> {
    pub process_name: Option<String>,
    pub process_id: ProcessId,
    pub status: RefCell<Status>,
    pub spec: S,
    pub model: RefCell<S::Model>,
    pub subscriber: Rc<Fn(Rc<Any>)->Option<S::Msg>>,
    pub offline_html: RefCell<HtmlBuild<S::Msg>>,
    pub online_html: LiveHtml<S::Msg>,
    pub queued_commands: Rc<RefCell<VecDeque<CmdRequest>>>,
}


