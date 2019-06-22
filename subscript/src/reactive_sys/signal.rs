use std::cell::*;
use std::rc::*;
use std::sync::mpsc;
use either::{Either, Either::*};

use crate::reactive_sys::value::*;


///////////////////////////////////////////////////////////////////////////////
// GENERIC INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub trait UnitSignal<T> {
    fn signal_output(&self) -> SignalOutput<T>;
    fn box_clone(&self) -> Box<UnitSignal<T>>;
}

impl<T: 'static> UnitSignal<T> for Signal<T> {
    fn signal_output(&self) -> SignalOutput<T> {SignalOutput(self.0.clone())}
    fn box_clone(&self) -> Box<UnitSignal<T>> {
        Box::new(self.clone())
    }
}
impl<T: 'static> UnitSignal<T> for SignalOutput<T> {
    fn signal_output(&self) -> SignalOutput<T> {self.clone()}
    fn box_clone(&self) -> Box<UnitSignal<T>> {
        Box::new(self.clone())
    }
}


///////////////////////////////////////////////////////////////////////////////
// SIGNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Signal<T>(Value<T>);

impl<T: 'static> Signal<T> {
    pub fn new(x: T) -> Self {
        Signal(Value::new_mutable(x))
    }
    pub fn get(&self) -> Rc<T> {
        self.0.get()
    }
    pub fn get_copy(&self) -> T where T: Clone {
        self.0.get().as_ref().clone()
    }
    pub fn set(&mut self, x: T) {
        self.0.set(x);
    }
    pub(crate) fn map<U: 'static>(&self, f: impl Fn(&T) -> U + 'static) -> SignalOutput<U> {
        SignalOutput(self.0.map(f))
    }
    pub(crate) fn zip<U: 'static>(&self, other: &UnitSignal<U>) -> SignalOutput<(T, U)>
    where
        T: Clone,
        U: Clone,
    {
        SignalOutput(self.0.zip(&other.signal_output().0))
    }
}



///////////////////////////////////////////////////////////////////////////////
// SIGNAL-OUTPUT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct SignalOutput<T>(pub(crate) Value<T>);

impl<T: 'static> SignalOutput<T> {
    pub fn get(&self) -> Rc<T> {
        self.0.get()
    }
    pub fn get_copy(&self) -> T where T: Clone {
        self.0.get().as_ref().clone()
    }
    pub(crate) fn map<U: 'static>(&self, f: impl Fn(&T) -> U + 'static) -> SignalOutput<U> {
        SignalOutput(self.0.map(f))
    }
    pub(crate) fn zip<U: 'static>(&self, other: &UnitSignal<U>) -> SignalOutput<(T, U)>
    where
        T: Clone,
        U: Clone,
    {
        SignalOutput(self.0.zip(&other.signal_output().0))
    }
}



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal(self.0.clone())
    }
}
impl<T> Clone for SignalOutput<T> {
    fn clone(&self) -> Self {
        SignalOutput(self.0.clone())
    }
}

impl<T: 'static +  Default> Default for Signal<T> {
    fn default() -> Self {
        Signal::new(Default::default())
    }
}
impl<T: 'static +  Default> Default for SignalOutput<T> {
    fn default() -> Self {
        SignalOutput(Value::new_static(Default::default()))
    }
}



