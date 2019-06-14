use std::collections::*;
use std::cell::*;
use std::marker::*;
use std::any::*;
use std::rc::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::instances::TickEnv;


pub trait Spec where Self: Clone {
	type Msg;
	type Model: Serialize + DeserializeOwned;
	
	fn init(&self, startup: StartupInfo<Self>) -> Init<Self>;
	fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &mut SubSystems<Self>);
	fn view(&self, model: &Self::Model) -> View<Self::Msg>;
}


#[derive(Debug)]
pub struct StartupInfo<S: Spec> {
	pub name: String,
	pub saved_model: Option<S::Model>,
}

pub struct Init<S: Spec> {
	pub model: S::Model,
	pub subs: Subscriptions<S::Msg>
}

impl<S: Spec> Default for Init<S>
where
    S::Model: Default,
{
    fn default() -> Self {
        Init {
            model: Default::default(),
            subs: Subscriptions::default(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// SUBSCRIPTIONS
///////////////////////////////////////////////////////////////////////////////


pub struct Subscriptions<Msg> {
    signal_sub: Vec<Box<Fn()->Option<Msg>>>,
    mail_subs: Vec<Box<Fn(Rc<Any>)->Option<Msg>>>,
}

impl<Msg> Default for Subscriptions<Msg> {
    fn default() -> Self {
        Subscriptions {
            signal_sub: Vec::new(),
            mail_subs: Vec::new(),
        }
    }
}

impl<Msg> Subscriptions<Msg> {
    pub fn add_signal_sub(&mut self, f: impl Fn() -> Option<Msg> + 'static) {
        self.signal_sub.push(Box::new(f));
    }
    pub fn add_mail_sub(&mut self, f: impl Fn(Rc<Any>) -> Option<Msg> + 'static) {
        self.mail_subs.push(Box::new(f));
    }
}

///////////////////////////////////////////////////////////////////////////////
// SUBSCRIPTIONS - TICK
///////////////////////////////////////////////////////////////////////////////


impl<Msg> Subscriptions<Msg> {
    pub(crate) fn tick<S: Spec + 'static>(&self, tick_env: &mut TickEnv<Msg>){
        let self_tid = TypeId::of::<S>();
        for f in self.signal_sub.iter() {
            if let Some(msg) = f() {
                tick_env.local_messages.push(msg);
            }
        }
        for message in tick_env.system_messages {
            let mut done: Option<Msg> = None;
            for f in self.mail_subs.iter() {
                apply_message::<S, Msg>(&mut tick_env.local_messages, f, message.clone());
            }
        }
    }
}

fn apply_message<S: Spec + 'static, Msg>(
    output: &mut Vec<Msg>,
    mail_sub: &Box<Fn(Rc<Any>)->Option<Msg>>,
    message: SystemMessage,
) {
    let self_tid = TypeId::of::<S>();
    let mut result: Option<Msg> = None;
    match message {
        SystemMessage::Private{from_tid, to_tid, value} => {
            if (from_tid != self_tid) && (to_tid == self_tid) {
                if let Some(msg) = mail_sub(value) {
                    output.push(msg);
                }
            }
        }
        SystemMessage::Public{from_tid, value} => {
            if from_tid != self_tid {
                if let Some(msg) = mail_sub(value) {
                    output.push(msg);
                }
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// GLOABL MESSAGES
///////////////////////////////////////////////////////////////////////////////
thread_local! {
    pub(crate) static GLOABL_MESSAGE_REGISTRY: RefCell<VecDeque<SystemMessage>> = {
        RefCell::new(VecDeque::new())
    };
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
    Public {
        from_tid: TypeId,
        value: Rc<Any>,
    },
    Private {
        from_tid: TypeId,
        to_tid: TypeId,
        value: Rc<Any>,
    },
}


///////////////////////////////////////////////////////////////////////////////
// NAVIGATION
///////////////////////////////////////////////////////////////////////////////

pub trait UrlPath {
    fn stringify(&self) -> String;
}

impl UrlPath for &str {
    fn stringify(&self) -> String {String::from(*self)}
}
impl UrlPath for String {
    fn stringify(&self) -> String {self.clone()}
}


///////////////////////////////////////////////////////////////////////////////
// SUB-SYSTEMS
///////////////////////////////////////////////////////////////////////////////

pub struct SubSystems<S: Spec> {
    pub(crate) requests: VecDeque<SystemRequest>,
    pub(crate) mark: PhantomData<S>,
}

#[derive(Debug)]
pub(crate) enum SystemRequest {
    Save,
    Message(SystemMessage),
    Navigate(String),
}


impl<S: Spec + 'static> SubSystems<S> {
	pub fn save(&mut self) {
        self.requests.push_back(SystemRequest::Save);
    }
    pub fn broadcast(&mut self, msg: impl Any) {
        self.requests.push_back(SystemRequest::Message(
            SystemMessage::Public {
                from_tid: TypeId::of::<S>(),
                value: Rc::new(msg),
            }
        ));
    }
	pub fn notify<T: Spec + 'static>(&mut self, msg: impl Any) {
        self.requests.push_back(SystemRequest::Message(SystemMessage::Private {
            from_tid: TypeId::of::<S>(),
            to_tid: TypeId::of::<T>(),
            value: Rc::new(msg)
        }));
    }
    pub fn navigate(&mut self, path: impl UrlPath) {
        self.requests.push_back(SystemRequest::Navigate(path.stringify()));
    }
}


///////////////////////////////////////////////////////////////////////////////
// SYSTEM-REQUEST HELPERS
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn process_system_requests<S: Spec + 'static>(name: &str, model: &S::Model, sys: &mut SubSystems<S>) {
    for msg in sys.requests.drain(..) {
        match msg {
            SystemRequest::Save => {
                save_model::<S>(name, model);
            }
            SystemRequest::Message(msg) => {
                register_message(msg);
            }
            SystemRequest::Navigate(nav) => {
                navigate(nav.as_str());
            }
        }
    }
}

pub(crate) fn spec_key<S: Spec + 'static>(name: &str) -> String {
    let tid = TypeId::of::<S>();
    format!("{:?}-{}", tid, name)
}

pub(crate) fn save_model<S: Spec + 'static>(name: &str, model: &S::Model) {
    browser::window()
        .local_storage
        .set::<S::Model>(&spec_key::<S>(name), model);
}

pub(crate) fn load_saved_model<S: Spec + 'static>(name: &str) -> Option<S::Model> {
    browser::window()
        .local_storage
        .get::<S::Model>(&spec_key::<S>(name))
}

pub(crate) fn register_message(msg: SystemMessage) {
    GLOABL_MESSAGE_REGISTRY.with(move |reg| {
        reg.borrow_mut().push_back(msg);
    });
}

pub(crate) fn navigate(route: &str) {
    browser::window()
        .history
        .push_state(route);
}





