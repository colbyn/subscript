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
// SPEC - CORE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct InitArgs<Model> {
    pub saved_model: Option<Model>
}

pub struct Init<Model, Msg> {
    pub model: Model,
    pub subs: Subscriptions<Msg>,
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
    
    fn new() -> Self;
    fn init(&self, loaded: InitArgs<Self::Model>, key: &InitKey) -> Init<Self::Model, Self::Msg>;
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, cmd: &Cmd);
    fn view(&self, model: &Self::Model) -> Html<Self::Msg>;
}


///////////////////////////////////////////////////////////////////////////////
// SPEC - COMMANDS
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone, Debug)]
pub enum CmdRequest {
    Save,
    Navigate(String),
    Broadcast(Rc<Any>),
}


///////////////////////////////////////////////////////////////////////////////
// SPEC - SUBSCRIPTIONS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Subscriptions<Msg> {
    pub global_events: Rc<Vec<Box<Fn(Rc<Any>)->Option<Msg>>>>,
    pub reactive_observers: Rc<Vec<Box<Fn()->Option<Msg>>>>,
}

///////////////////////////////////////////////////////////////////////////////
// SPEC - SUBSCRIPTIONS - REACTIVE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct InitKey(pub(crate) ());

#[derive(Clone)]
pub struct Reactive<T>(pub(crate) ReactiveValue<T>);

#[derive(Clone)]
pub enum ReactiveValue<T> {
    JsHandler {
        value: Rc<RefCell<T>>,
        listener: Callback<T>,
    },
    Mutable {
        v1: Rc<RefCell<T>>,
        v2: Rc<RefCell<T>>,
    }
}




///////////////////////////////////////////////////////////////////////////////
// INTERNAL GLOBAL REGISTRY
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_REGISTRY: GlobalRegistry = {
        GlobalRegistry {
            events: RefCell::new(Vec::new()),
            processes: RefCell::new(Vec::new()),
        }
    };
}

pub struct GlobalRegistry {
    pub events: RefCell<Vec<Rc<Any>>>,
    pub processes: RefCell<Vec<Box<ProcessHandle>>>,
}



///////////////////////////////////////////////////////////////////////////////
// APPLICATION
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct AppBuilder {
    pub process: Rc<ProcessHandle>,
}


#[derive(Clone)]
pub struct Application {
    pub js_tick_callback: Rc<RefCell<Option<Callback<()>>>>,
    pub root_process: Rc<ProcessHandle>,
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

pub trait ProcessHandle {
    fn spec_type_id(&self) -> TypeId;
    fn process_id(&self) -> String;
    fn dom_ref(&self) -> &DomRef;
    fn start(&self);
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
    pub model: RefCell<S::Model>,
    pub subs: Subscriptions<S::Msg>,
    pub offline_html: RefCell<HtmlBuild<S::Msg>>,
    pub online_html: LiveHtml<S::Msg>,
    pub queued_commands: Rc<RefCell<VecDeque<CmdRequest>>>,
}


// impl<S: Spec> PartialEq for Process<S> {
// 
// }

