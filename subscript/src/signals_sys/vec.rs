pub mod view_observer;

use std::fmt::Debug;
use std::marker::*;
use std::any::*;
use std::cell::*;
use std::rc::*;
use std::collections::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};


///////////////////////////////////////////////////////////////////////////////
// SIGNAL-OBSERVERS
///////////////////////////////////////////////////////////////////////////////
pub trait VecObserver<T> {
    fn push_op(&mut self, new: &T);
    fn insert_op(&mut self, ix: usize, new: &T);
    fn remove_op(&mut self, ix: usize);
}

// impl<Msg> std::fmt::Debug for VecSignal<Msg> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "EventHandler")
//     }
// }


///////////////////////////////////////////////////////////////////////////////
// SIGNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct VecSignal<T> {
    value: Vec<T>,
    #[serde(skip)]
    observers: RefCell<Vec<Box<VecObserver<T>>>>,
}

impl<T> VecSignal<T> {
    pub fn new() -> Self {
        VecSignal {
            value: Vec::new(),
            observers: RefCell::new(Vec::new()),
        }
    }
    pub fn push(&mut self, value: T) {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.push_op(&value);
        }
        self.value.push(value);
    }
    pub fn insert(&mut self, ix: usize, value: T) {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.insert_op(ix, &value);
        }
        self.value.insert(ix, value);
    }
}

impl<T: Default> Default for VecSignal<T> {
    fn default() -> Self {
        VecSignal {
            value: Vec::new(),
            observers: RefCell::new(Vec::new())
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// SIGNAL - INTERNAL API
///////////////////////////////////////////////////////////////////////////////

impl<T> VecSignal<T> {
    pub(crate) fn add_observer(&self, new: impl VecObserver<T> + 'static) {
        self.observers.borrow_mut().push(Box::new(new));
    }
}

