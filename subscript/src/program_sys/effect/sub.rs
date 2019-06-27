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
use crate::reactive_sys::*;
use crate::program_sys::spec::Spec;

pub use crate::program_sys::shell::{Shell};
pub use crate::program_sys::effect::nav::UrlString;


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

impl<Msg: 'static> Subscriptions<Msg> {
    pub fn add_signal_sub<S: Reactive<T>, T: 'static + PartialEq + Clone>(
        &mut self,
        signal: &S,
        on_change: impl Fn(T) -> Msg + 'static,
    ) {
        let f: Box<Fn() -> Option<Msg> + 'static> = Box::new({
            let signal = signal.signal_output();
            let recorded_value: Rc<RefCell<Rc<T>>> = Rc::new(RefCell::new(signal.get()));
            move || -> Option<Msg> {
                let mut result: Option<Msg> = None;
                let signal = signal.clone();
                let current_value: Rc<T> = signal.get();
                let unchanged = {
                    let x: &T = current_value.as_ref();
                    let y: &T = &recorded_value.borrow();
                    x == y
                };
                if !unchanged {
                    result = Some(on_change(signal.get_copy()));
                    recorded_value.replace(current_value);
                }
                result
            }
        });
        self.signal_sub.push(Box::new(f));
    }
    pub fn add_msg_sub<T: 'static + Clone>(&mut self, on_global_message: impl Fn(T) -> Msg + 'static) {
        let f: Box<Fn(Rc<Any>)->Option<Msg>> = Box::new({
            move |something: Rc<Any>| -> Option<Msg> {
                let mut result: Option<Msg> = None;
                if let Some(value) = something.downcast_ref::<T>() {
                    result = Some(on_global_message(value.clone()));
                }
                result
            }
        });
        self.mail_subs.push(f);
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

///////////////////////////////////////////////////////////////////////////////
// SUBSCRIPTIONS - MACRO
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! i_subs_entry {
    ($subs:expr; sig $name:ident ($value:expr => $new_value:ident) -> $msg:ty {$($body:tt)*}) => {{
        $subs.add_signal_sub(&$value, move |$new_value| -> $msg {{
            $($body)*
        }});
    }};
    ($subs:expr; msg $name:ident ($value:ident : $type:ty) -> $msg:ty {$body:expr}) => {{
        $subs.add_msg_sub(move |$value : $type| {
            $body
        });
    }};
}

#[macro_export]
macro_rules! subs {
    () => {{
        use ::subscript::program_sys::spec::*;
        let subs = Subscriptions::default();
        subs
    }};
    ($($kind:ident $fn_name:ident $args:tt -> $msg:ty {$($body:tt)*})*) => {{
        use std::any::{Any, TypeId};
        use std::rc::Rc;
        use ::subscript::program_sys::spec::*;

        let mut subs = Subscriptions::default();
        $({
            i_subs_entry!(subs; $kind $fn_name $args -> $msg {{$($body)*}});
        })*
        subs
    }};
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

// use crate::program_sys::spec::*;

// enum Msg {
//     NoOp,
//     UrlRequest(String),
//     UrlChanged(UrlRequest),
// }

// #[derive(Clone, PartialEq)]
// pub struct UrlRequest(String);
// type UrlSignal = Signal<UrlRequest>;

// pub struct App {
//     url: UrlSignal
// }

// pub fn run(this: &App) {
//     let subs: Subscriptions<Msg> = subs!{
//         sig url_changed(this.url => new_url) -> Msg {
//             Msg::UrlChanged(new_url)
//         }
//         msg url_request(value: UrlRequest) -> Msg {
//             Msg::UrlRequest(value.0)
//         }
//     };
// }
