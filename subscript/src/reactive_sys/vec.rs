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

///////////////////////////////////////////////////////////////////////////////
// SIGNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct VecSignal<T> {
    value: Rc<RefCell<Vec<T>>>,
    #[serde(skip)]
    observers: Rc<RefCell<Vec<Box<VecObserver<T>>>>>,
}

impl<T> VecSignal<T> {
    pub fn new() -> Self {
        VecSignal {
            value: Rc::new(RefCell::new(Vec::new())),
            observers: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn push(&mut self, value: T) {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.push_op(&value);
        }
        self.value.borrow_mut().push(value);
    }
    pub fn insert(&mut self, ix: usize, value: T) {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.insert_op(ix, &value);
        }
        self.value.borrow_mut().insert(ix, value);
    }
    pub fn update_by(&mut self, pred: impl Fn(&T)->bool, f: impl FnMut(&mut T)) {
        let pos = self.value.borrow().iter().position(|x| pred(x));
        match pos {
            None => {}
            Some(ix) => {
                let mut f = Box::new(f);
                f(self.value.borrow_mut().get_mut(ix).expect("update_by internal error"));
            }
        }
    }
    pub fn inspect<U>(&self, f: impl Fn(&Vec<T>)->U) -> U {
        f(&self.value.borrow())
    }
}

impl<T: Default> Default for VecSignal<T> {
    fn default() -> Self {
        VecSignal {
            value: Rc::new(RefCell::new(Vec::new())),
            observers: Rc::new(RefCell::new(Vec::new()))
        }
    }
}
impl<T> Clone for VecSignal<T> {
    fn clone(&self) -> Self {
        let value = self.value.clone();
        let observers = self.observers.clone();
        VecSignal{value, observers}
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
