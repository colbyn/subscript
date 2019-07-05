pub mod view_observer;

use std::fmt::Debug;
use std::marker::*;
use std::any::*;
use std::cell::*;
use std::rc::*;
use std::collections::*;

use crate::reactive_sys::signal::{Signal, Formula, Reactive};
use crate::reactive_sys::value::*;
use crate::reactive_sys::value;

///////////////////////////////////////////////////////////////////////////////
// SIGNAL-OBSERVERS
///////////////////////////////////////////////////////////////////////////////
pub trait VecOpObserver<T> {
    fn push_op(&mut self, new: &T);
    fn insert_op(&mut self, ix: usize, new: &T);
    fn remove_op(&mut self, ix: usize);
}


///////////////////////////////////////////////////////////////////////////////
// SIGNAL
///////////////////////////////////////////////////////////////////////////////

pub struct VecSignal<T> {
    pub(crate) value: Rc<RefCell<Vec<T>>>,
    pub(crate) ops_subscribers: Rc<RefCell<Vec<Box<VecOpObserver<T>>>>>,
    pub(crate) change_subscribers: Rc<RefCell<Vec<Box<FnMut(&Vec<T>)>>>>,
}

impl<T> VecSignal<T> {
    pub fn new() -> Self {
        VecSignal {
            value: Rc::new(RefCell::new(Vec::new())),
            ops_subscribers: Rc::new(RefCell::new(Vec::new())),
            change_subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn push(&mut self, value: T) {
        for sub in self.ops_subscribers.borrow_mut().iter_mut() {
            sub.push_op(&value);
        }
        self.value.borrow_mut().push(value);
        for sub in self.change_subscribers.borrow_mut().iter_mut() {
            sub(&self.value.borrow());
        }
    }
    pub fn insert(&mut self, ix: usize, value: T) {
        for sub in self.ops_subscribers.borrow_mut().iter_mut() {
            sub.insert_op(ix, &value);
        }
        self.value.borrow_mut().insert(ix, value);
        for sub in self.change_subscribers.borrow_mut().iter_mut() {
            sub(&self.value.borrow());
        }
    }
    pub fn remove(&self, ix: usize) {
        for sub in self.ops_subscribers.borrow_mut().iter_mut() {
            sub.remove_op(ix);
        }
        self.value.borrow_mut().remove(ix);
        for sub in self.change_subscribers.borrow_mut().iter_mut() {
            sub(&self.value.borrow());
        }
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
    pub fn remove_by(&mut self, pred: impl Fn(&T)->bool) {
        let pos = self.value.borrow().iter().position(|x| pred(x));
        match pos {
            None => {}
            Some(ix) => {
                self.remove(ix);
            }
        }
    }
    pub fn get_by<U>(&self, f: impl Fn(&Vec<T>)->U) -> U {
        f(&self.value.borrow())
    }
    ///////////////////////////////////////////////////////////////////////////
    // OUTPUT-STREAM OPS
    ///////////////////////////////////////////////////////////////////////////
    pub fn reduce_to<U: 'static>(&self, f: impl Fn(&Vec<T>) -> U + 'static) -> Formula<U> {
        let subscribers: value::SubscribersRef<U> =
            value::SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let apply = move |x: &Vec<T>| -> Rc<U> {
            Rc::new(f(x))
        };
        let current_value: Rc<RefCell<Rc<U>>> = Rc::new(RefCell::new(apply(&self.value.borrow())));
        let getter: Rc<Fn()->Rc<U>> = Rc::new({
            let current_value = current_value.clone();
            move || {
                current_value.borrow().clone()
            }
        });
        let result = value::Value{subscribers, getter, setter: None};
        self.change_subscribers.borrow_mut().push(Box::new({
            let current_value = current_value.clone();
            let result = result.clone();
            move |new_value: &Vec<T>| {
                let result = result.clone();
                current_value.replace(apply(new_value));
                result.notify_subscribers(&current_value.borrow());
            }
        }));
        Formula(result)
    }
    pub fn traverse_to<U: 'static>(
        &self,
        f: impl Fn(&T) -> U + 'static,
    ) -> Formula<Vec<U>> {
        let subscribers: value::SubscribersRef<Vec<U>> =
            value::SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let apply = move |xs: &Vec<T>| -> Rc<Vec<U>> {
            let inner =  xs
                .iter()
                .map(|x| f(x))
                .collect::<Vec<_>>();
            Rc::new(inner)
        };
        let current_value: Rc<RefCell<Rc<Vec<U>>>> = Rc::new(RefCell::new(
            apply(&self.value.borrow())
        ));
        let getter: Rc<Fn()->Rc<Vec<U>>> = Rc::new({
            let current_value = current_value.clone();
            move || {
                current_value.borrow().clone()
            }
        });
        let result = value::Value{subscribers, getter, setter: None};
        self.change_subscribers.borrow_mut().push(Box::new({
            let current_value = current_value.clone();
            let result = result.clone();
            move |new_value: &Vec<T>| {
                let result = result.clone();
                current_value.replace(apply(new_value));
                result.notify_subscribers(&current_value.borrow());
            }
        }));
        Formula(result)
    }
    pub(crate) fn add_observer(&self, new: impl VecOpObserver<T> + 'static) {
        self.ops_subscribers.borrow_mut().push(Box::new(new));
    }
}

impl<T> Default for VecSignal<T> {
    fn default() -> Self {
        VecSignal {
            value: Rc::new(RefCell::new(Vec::new())),
            ops_subscribers: Rc::new(RefCell::new(Vec::new())),
            change_subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl<T> Clone for VecSignal<T> {
    fn clone(&self) -> Self {
        let value = self.value.clone();
        let ops_subscribers = self.ops_subscribers.clone();
        let change_subscribers = self.change_subscribers.clone();
        VecSignal{value, ops_subscribers, change_subscribers}
    }
}



///////////////////////////////////////////////////////////////////////////////
// READ-ONLY VEC-SIGNAL
///////////////////////////////////////////////////////////////////////////////

pub struct VecFormula<T>(pub(crate) VecSignal<T>);

impl<T> VecFormula<T> {
    pub(crate) fn reduce_to<U: 'static>(&self, f: impl Fn(&Vec<T>) -> U + 'static) -> Formula<U> {
        self.0.reduce_to(f)
    }
    pub(crate) fn traverse_to<U: 'static>(&self, f: impl Fn(&T) -> U + 'static) -> Formula<Vec<U>> {
        self.0.traverse_to(f)
    }
}

impl<T: Default> Default for VecFormula<T> {
    fn default() -> Self {
        VecFormula(VecSignal::default())
    }
}
impl<T> Clone for VecFormula<T> {
    fn clone(&self) -> Self {
        VecFormula(self.0.clone())
    }
}
