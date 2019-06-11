use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};


///////////////////////////////////////////////////////////////////////////////
// CELLS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Signal<T> {
	pub(crate) value: Rc<RefCell<T>>,
	pub(crate) subscribers: Rc<RefCell<Vec<SignalSub<T>>>>
}

#[derive(Debug)]
pub struct SignalSub<T> {
	pub(crate) value: Rc<RefCell<T>>,
	pub(crate) status: Rc<RefCell<Status>>
}

impl<T> Clone for Signal<T> {
	fn clone(&self) -> Self {
		Signal {
			value: self.value.clone(),
			subscribers: self.subscribers.clone(),
		}
	}
}
impl<T> Clone for SignalSub<T> {
	fn clone(&self) -> Self {
		SignalSub {
			value: self.value.clone(),
			status: self.status.clone(),
		}
	}
}
#[derive(Debug, PartialEq)]
pub enum Status {
	Unchanged,
	Changed,
}

impl<T> Signal<T> {
	pub fn new(value: T) -> Self {
		let value = Rc::new(RefCell::new(value));
		let subscribers = Rc::new(RefCell::new(Vec::new())); 
		Signal {value,subscribers}
	}
	pub fn set(&mut self, value: T) {
		for sub in self.subscribers.borrow_mut().iter_mut() {
			sub.status.replace(Status::Changed);
		}
		self.value.replace(value);
	}
	pub fn get(&mut self) -> &T {
		unimplemented!()
	}
	pub(crate) fn new_subscriber(&self) -> SignalSub<T> {
		let sub = SignalSub {
			value: self.value.clone(),
			status: Rc::new(RefCell::new(Status::Unchanged)),
		};
		self.subscribers.borrow_mut().push(sub.clone());
		sub
	}
}


impl<T> SignalSub<T> {
	pub(crate) fn changed(&self) -> bool {
		let status: &Status = &self.status.borrow();
		status == &Status::Changed
	}
	pub(crate) fn reset(&self) {
		self.status.replace(Status::Unchanged);
	}
	pub(crate) fn if_changed(&self, mut f: impl FnMut(&T)) {
		if self.changed() {
			f(&self.value.borrow());
			self.reset();
		}
	}
}
