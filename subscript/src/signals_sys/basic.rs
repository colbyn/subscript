pub mod observers;

use core::fmt::Debug;
use std::marker::*;
use std::any::*;
use std::cell::*;
use std::rc::*;
use std::collections::*;



///////////////////////////////////////////////////////////////////////////////
// SIGNAL-OBSERVERS
///////////////////////////////////////////////////////////////////////////////
pub trait SignalObserver<T> {
    fn set_op(&mut self, new: &T);
}


///////////////////////////////////////////////////////////////////////////////
// SIGNAL
///////////////////////////////////////////////////////////////////////////////

pub struct Signal<T> {
    pub(crate) value: T,
    observers: RefCell<Vec<Box<SignalObserver<T>>>>,
}

impl<T: Default> Default for Signal<T> {
    fn default() -> Self {
        Signal {
            value: Default::default(),
            observers: RefCell::new(Vec::new()),
        }
    }
}

impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Signal{value, observers: RefCell::new(Vec::new())}
    }
    pub fn set(&mut self, new: T) {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.set_op(&new);
        }
        self.value = new;
    }
    pub fn get_clone(&self) -> T where T: Clone {
        self.value.clone()
    }
}

impl Signal<String> {
    pub fn get(&self) -> &String {
        &self.value
    }
}

///////////////////////////////////////////////////////////////////////////////
// SIGNAL - INTERNAL API
///////////////////////////////////////////////////////////////////////////////

impl<T> Signal<T> {
    pub(crate) fn add_observer(&self, new: impl SignalObserver<T> + 'static) {
        self.observers.borrow_mut().push(Box::new(new));
    }
}

