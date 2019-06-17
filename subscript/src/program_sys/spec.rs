use std::collections::*;
use std::cell::*;
use std::marker::*;
use std::any::*;
use std::rc::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::backend::browser;
use crate::view_sys::dsl::View;
use crate::program_sys::instances::TickEnv;
use crate::program_sys::shell::*;

pub use crate::program_sys::shell::{Shell};
pub use crate::program_sys::effect::nav::UrlPath;


pub trait Spec where Self: Clone {
	type Msg;
    type Model;
	
	fn init(&self, startup: StartupInfo<Self>) -> Init<Self>;
	fn update(&self, model: &mut Self::Model, msg: Self::Msg, sh: &mut Shell<Self>);
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
    pub(crate) fn tick<S: Spec + 'static>(&self, instance_name: &str, tick_env: &mut TickEnv<Msg>){
        let self_tid = TypeId::of::<S>();
        for f in self.signal_sub.iter() {
            if let Some(msg) = f() {
                tick_env.local_messages.push(msg);
            }
        }
        for message in tick_env.system_messages {
            let sender_is_receiver = message.sender_is_receiver::<S>(instance_name);
            let opt_valid_private_address = {
                let mut result = true;
                if let Some(to_tid) = message.is_private() {
                    result = to_tid == self_tid;
                }
                result
            };
            if (!sender_is_receiver) && opt_valid_private_address {
                for mail_sub in self.mail_subs.iter() {
                    if let Some(msg) = mail_sub(message.value()) {
                        tick_env.local_messages.push(msg);
                    }
                }
            }
        }
    }
}




