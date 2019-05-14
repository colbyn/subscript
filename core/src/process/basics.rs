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

impl<Model: Default, Msg> Default for Init<Model, Msg> {
    fn default() -> Self {
        Init {
            model: Default::default(),
            subs: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Cmd {
    pub queued_commands: Rc<RefCell<VecDeque<CmdRequest>>>,
}

pub type Html<Msg> = HtmlBuild<Msg>;

pub trait Spec
where
    Self: Clone + 'static
{
    type Msg: Debug + Clone + 'static;
    type Model: Debug + Clone + Serialize + DeserializeOwned + PartialEq;
    
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
    pub fn set(&self, new: T) -> Self {
        match &self.0 {
            ReactiveValue::Mutable{v2, ..} => {
                v2.replace(new);
            },
            _ => ()
        }
        self.clone()
    }
}


///////////////////////////////////////////////////////////////////////////////
// SPEC
///////////////////////////////////////////////////////////////////////////////

impl Cmd {
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
    ($sb:expr; on($self:ident . $react:ident -> $new:ident) -> $msg:ty {$($xs:tt)*} $($rest:tt)*) => {{
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
        use crate::process::basics::*;
        use crate::process::online::*;
        
        Subscriptions {
            global_events: Rc::new(Vec::new()),
            reactive_observers: Rc::new(Vec::new()),
        }
    }};
    ($($xs:tt)*) => {{
        use crate::process::basics::*;
        use crate::process::online::*;
        
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











