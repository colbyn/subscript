use core::marker::PhantomData;
use std::rc::*;
use std::any::*;
use std::cell::*;
use std::collections::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Function;

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::instances::TickEnv;
use crate::program_sys::spec::Spec;
use crate::program_sys::effect::nav::*;


///////////////////////////////////////////////////////////////////////////////
// SHELL
///////////////////////////////////////////////////////////////////////////////


/// It’s a reincarnated-bourne-again shell for your everyday web-app
/// needs. :)
///
/// User-level commands are exposed or rather implemented as methods on
/// the `Shell` type (so from your docs navigate to “methods” section).
pub struct Shell<S: Spec> {
    pub(crate) instance_name: String,
    pub(crate) commands: RefCell<VecDeque<Command>>,
    pub(crate) mark: PhantomData<S>,
    // pub(crate) timeouts: Vec<>,
}

pub(crate) enum Command {
    Save,
    Message(SystemMessage),
    Navigate(String),
}


impl<S: Spec + 'static> Shell<S> {
    pub fn save(&mut self) {
        self.commands.borrow_mut().push_back(Command::Save);
    }
    pub fn broadcast(&mut self, msg: impl Any) {
        self.commands.borrow_mut().push_back(Command::Message(
            SystemMessage::Public {
                from_name: self.instance_name.clone(),
                from_tid: TypeId::of::<S>(),
                value: Rc::new(msg),
            }
        ));
    }
    pub fn message<T: Spec + 'static, V: Any>(&mut self, msg: V) {
        let from_tid = TypeId::of::<S>();
        let to_tid = TypeId::of::<T>();
        self.commands.borrow_mut().push_back(Command::Message(SystemMessage::Private {
            from_name: self.instance_name.clone(),
            from_tid,
            to_tid,
            value: Rc::new(msg)
        }));
    }
    pub fn navigate(&mut self, path: impl UrlString) {
        self.commands.borrow_mut().push_back(Command::Navigate(path.url_string()));
    }
    pub fn current_url(&self) -> Url {
        Url::get_current(&browser::window())
    }
    pub fn cache(&self) -> Cache {
        Cache(())
    }
}

///////////////////////////////////////////////////////////////////////////////
// CACHE
///////////////////////////////////////////////////////////////////////////////

pub struct Cache(pub(crate) ());

impl Cache {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        browser::window()
            .local_storage
            .get::<T>(key)
    }
    pub fn insert<T: Serialize>(&self, key: &str, value: &T) {
        browser::window()
            .local_storage
            .set::<T>(key, value);
    }
    pub fn remove(&self, key: &str) {
        browser::window()
            .local_storage
            .remove(key);
    }
}

///////////////////////////////////////////////////////////////////////////////
// TIMEOUT
///////////////////////////////////////////////////////////////////////////////

// struct Timeouts<Msg>(Vec<Timeout<Msg>>);
// struct Timeout<Msg>{
//     triggered: RefCell<bool>,
//     name: Option<String>,
//     on_timeout: Box<Fn()->Msg>,
// }


///////////////////////////////////////////////////////////////////////////////
// GLOABL MESSAGES
///////////////////////////////////////////////////////////////////////////////
thread_local! {
    pub(crate) static GLOABL_MESSAGE_REGISTRY: RefCell<VecDeque<SystemMessage>> = {
        RefCell::new(VecDeque::new())
    };
}

#[derive(Debug, Clone)]
pub(crate) enum SystemMessage {
    Public {
        from_name: String,
        from_tid: TypeId,
        value: Rc<Any>,
    },
    Private {
        from_name: String,
        from_tid: TypeId,
        to_tid: TypeId,
        value: Rc<Any>,
    },
}

impl SystemMessage {
    pub fn is_private(&self) -> Option<TypeId> {
        match self {
            SystemMessage::Private{to_tid, ..} => Some(to_tid.clone()),
            _ => None
        }
    }
    pub(crate) fn value(&self) -> Rc<Any> {
        match self {
            SystemMessage::Private{value, ..} => value.clone(),
            SystemMessage::Public{value, ..} => value.clone(),
        }
    }
    pub(crate) fn from_name(&self) -> String {
        match self {
            SystemMessage::Private{from_name, ..} => from_name.clone(),
            SystemMessage::Public{from_name, ..} => from_name.clone(),
        }
    }
    pub(crate) fn from_tid(&self) -> TypeId {
        match self {
            SystemMessage::Private{from_tid, ..} => from_tid.clone(),
            SystemMessage::Public{from_tid, ..} => from_tid.clone(),
        }
    }
    pub(crate) fn sender_is_receiver<T: Spec + 'static>(&self, this_name: &str) -> bool {
        let this_tid = TypeId::of::<T>();
        let this_name = String::from(this_name);
        (self.from_name() == this_name) && (self.from_tid() == this_tid)
    }
}



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS HELPERS
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn process_system_requests<S: Spec + 'static>(name: &str, model: &S::Model, sys: &mut Shell<S>) {
    for msg in sys.commands.borrow_mut().drain(..) {
        match msg {
            Command::Save => {
                // save_model::<S>(name, model);
                unimplemented!()
            }
            Command::Message(msg) => {
                register_message(msg);
            }
            Command::Navigate(nav) => {
                navigate(nav.as_str());
                crate::program_sys::CURRENT_URL.with(|cell| {
                    let new_url = Url::get_current(&browser::window());
                    cell.replace(Some(new_url));
                });
            }
        }
    }
}

pub(crate) fn spec_key<S: Spec + 'static>(name: &str) -> String {
    let tid = TypeId::of::<S>();
    format!("{:?}-{}", tid, name)
}

pub(crate) fn save_model<S: Spec + 'static>(name: &str, model: &S::Model) {
    unimplemented!()
    // browser::window()
    //     .local_storage
    //     .set::<S::Model>(&spec_key::<S>(name), model);
}

pub(crate) fn load_saved_model<S: Spec + 'static>(name: &str) -> Option<S::Model> {
    unimplemented!()
    // browser::window()
    //     .local_storage
    //     .get::<S::Model>(&spec_key::<S>(name))
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