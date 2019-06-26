use std::cell::*;
use std::rc::*;
use std::sync::mpsc;
use either::{Either, Either::*};

use crate::reactive_sys::value::*;
use crate::reactive_sys::vec::VecFormula;


///////////////////////////////////////////////////////////////////////////////
// GENERIC INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub trait UnitSignal<T> {
    fn signal_output(&self) -> Formula<T>;
    fn box_clone(&self) -> Box<UnitSignal<T>>;
}

impl<T: 'static> UnitSignal<T> for Signal<T> {
    fn signal_output(&self) -> Formula<T> {Formula(self.0.clone())}
    fn box_clone(&self) -> Box<UnitSignal<T>> {
        Box::new(self.clone())
    }
}
impl<T: 'static> UnitSignal<T> for Formula<T> {
    fn signal_output(&self) -> Formula<T> {self.clone()}
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
    pub(crate) fn map<U: 'static>(&self, f: impl Fn(&T) -> U + 'static) -> Formula<U> {
        Formula(self.0.map(f))
    }
    pub(crate) fn zip<U: 'static>(&self, other: &UnitSignal<U>) -> Formula<(T, U)>
    where
        T: Clone,
        U: Clone,
    {
        Formula(self.0.zip(&other.signal_output().0))
    }
}



///////////////////////////////////////////////////////////////////////////////
// FORMULA - READ-ONLY SIGNALS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Formula<T>(pub(crate) Value<T>);

impl<T: 'static> Formula<T> {
    pub fn get(&self) -> Rc<T> {
        self.0.get()
    }
    pub fn get_copy(&self) -> T where T: Clone {
        self.0.get().as_ref().clone()
    }
    pub(crate) fn map<U: 'static>(&self, f: impl Fn(&T) -> U + 'static) -> Formula<U> {
        Formula(self.0.map(f))
    }
    pub(crate) fn zip<U: 'static>(&self, other: &UnitSignal<U>) -> Formula<(T, U)>
    where
        T: Clone,
        U: Clone,
    {
        Formula(self.0.zip(&other.signal_output().0))
    }
}

impl<T: 'static + Clone> Formula<Vec<T>> {
    pub fn to_vec_formula(&self) -> VecFormula<T> {
        use crate::reactive_sys::vec::*;
        let value: Rc<RefCell<Vec<T>>> = Rc::new(RefCell::new(self.get_copy()));
        let ops_subscribers: Rc<RefCell<Vec<Box<VecObserver<T>>>>> = Default::default();
        let change_subscribers: Rc<RefCell<Vec<Box<FnMut(&Vec<T>)>>>> = Default::default();
        let mut vec_signal = VecSignal{value,ops_subscribers,change_subscribers};
        VecFormula(vec_signal)
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
impl<T> Clone for Formula<T> {
    fn clone(&self) -> Self {
        Formula(self.0.clone())
    }
}

impl<T: 'static +  Default> Default for Signal<T> {
    fn default() -> Self {
        Signal::new(Default::default())
    }
}
impl<T: 'static +  Default> Default for Formula<T> {
    fn default() -> Self {
        Formula(Value::new_static(Default::default()))
    }
}



