use std::hash::Hash;
use std::any::*;
use std::marker::PhantomData;
use std::fmt::{self, Debug};
use std::cell::*;
use std::rc::*;
use std::collections::*;
use either::{Either, Left, Right};
use wasm_bindgen::JsValue;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use ss_web_utils::dom;
use ss_web_utils::prelude::*;
use ss_web_utils::js::{self, console};
use ss_view_tree::*;


pub trait Spec
where Self: std::marker::Sized + PartialEq + Debug + Clone
{
    type Model: Debug + PartialEq + Hash + Serialize + DeserializeOwned;
    type Msg: Debug + PartialEq + Clone;
    
    fn init(&self, startup: StartupInfo<Self>) -> Init<Self>;
    fn update(&self, model: &mut Self::Model, msg: Self::Msg, sys: &SubSystems);
    fn view(&self, model: &Self::Model) -> View<Self::Msg>;
}


#[derive(Debug)]
pub struct StartupInfo<S: Spec> {
	pub name: String,
	pub saved_model: Option<S::Model>,
}

#[derive(Debug)]
pub struct Init<S: Spec> {
	pub model: S::Model,
	pub subs: Subscriptions<S::Msg>
}

#[derive(Debug, PartialEq)]
pub struct Subscriptions<Msg> {
	listener: fn(Rc<Any>)->Option<Msg>,
}

impl<Msg> Default for Subscriptions<Msg> {
	fn default() -> Self {
		Subscriptions {
			listener: |_| None,
		}
	}
}

impl<Msg> Subscriptions<Msg> {
    pub fn tick(&self, messages: &mut Vec<Msg>, global_events: &Vec<Rc<Any>>) {
        for event in global_events.clone() {
            if let Some(msg) = (self.listener)(event) {
                messages.push(msg);
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// SUB-SYSTEMS
///////////////////////////////////////////////////////////////////////////////

pub trait Uri {
	fn stringify(&self) -> String;
}

impl Uri for str {
	fn stringify(&self) -> String {
		String::from(self)
	}
}


/// External API for interacting with internal framework facilities.
#[derive(Debug, PartialEq)]
pub struct SubSystems {
	pub(crate) requests: RefCell<VecDeque<SystemRequest>>,
}

impl Default for SubSystems {
    fn default() -> Self {
        SubSystems {
            requests: RefCell::new(VecDeque::new()),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SystemRequest {
	Save,
	Broadcast(Rc<Any>),
	Navigate(String),
}

impl PartialEq for SystemRequest {
    fn eq(&self, other: &SystemRequest) -> bool {
        match (self, other) {
            (SystemRequest::Save, SystemRequest::Save) => true,
            (SystemRequest::Broadcast(_), SystemRequest::Broadcast(_)) => true,
            (SystemRequest::Navigate(_), SystemRequest::Navigate(_)) => true,
            _ => false
        }
    }
}


impl SubSystems {
    pub fn navigate(&self, route: impl Uri) {
        self.requests.borrow_mut().push_back(
            SystemRequest::Navigate(route.stringify())
        );
    }
    pub fn save(&self) {
        self.requests.borrow_mut().push_back(
            SystemRequest::Save
        );
    }
    pub fn broadcast(&self, value: impl Any) {
        self.requests.borrow_mut().push_back(
            SystemRequest::Broadcast(Rc::new(value))
        );
    }
}