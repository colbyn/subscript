pub mod vec;

pub use vec::*;

// use std::collections::*;
// use std::any::*;
// use std::marker::*;
// use std::cell::*;
// use std::rc::*;
// use either::{Either, Either::*};
// use ss_web_utils::{dom, js, js::console, prelude::*};
// use crate::view::{dsl::View, dom::Dom};


// ///////////////////////////////////////////////////////////////////////////////
// // VECTORS
// ///////////////////////////////////////////////////////////////////////////////

// pub struct IVec<T, Msg>{
// 	pub(crate) data: Rc<RefCell<Vec<T>>>,
// 	pub(crate) subscribers: Rc<RefCell<Vec<IVecSubscriber<T, Msg>>>>,
// }
// pub struct IVecSubscriber<T, Msg> {
// 	pub(crate) provision: Box<FromIVec<T, Msg>>,
// 	pub(crate) updater: Updater<Msg>,
// }



// pub(crate) struct Updater<Msg>(pub Rc<RefCell<Linked<Msg>>>);
// impl<Msg> Updater<Msg> {
// 	fn push(&self, value: View<Msg>) {
// 		unimplemented!()
// 	}
// 	fn insert(&self, ix: usize, value: View<Msg>) {
// 		unimplemented!()
// 	}
// 	fn delete(&self, ix: usize) {
// 		unimplemented!()
// 	}
// 	pub(crate) fn new(segment: Vec<View<Msg>>) -> Self {
// 		Updater(Rc::new(RefCell::new(Linked::New(NewLinked {
// 			segment: segment,
// 			removed: Vec::new(),
// 		}))))
// 	}
// 	pub(crate) fn build(&self, f: impl Fn(Vec<View<Msg>>)->Vec<Dom<Msg>>) {
// 		let mut results = Vec::new();
// 		{
// 			let inner: &mut Linked<Msg> = &mut self.0.borrow_mut();
// 			match inner {
// 				Linked::New(NewLinked{segment, ..}) => {
// 					let children = segment.drain(..).collect::<Vec<_>>();
// 					results = f(children);
// 				}
// 				_ => panic!()
// 			}
// 		}
// 		self.0.replace(Linked::Dom(DomLinked {
// 			segment: results,
// 			removed: Vec::new(),
// 		}));
// 	}
// 	pub(crate) fn terminate(self) -> Linked<Msg> {
// 		unimplemented!()
// 		// if let Ok(inner) = Rc::try_unwrap(self.0) {
// 		// 	inner.into_inner()
// 		// }
// 		// else {panic!()}
// 	}
// }
// pub trait FromIVec<T, Msg> {
// 	fn new(&self, value: &T) -> View<Msg>;
// }
// impl<T, Msg> FromIVec<T, Msg> for Fn(&T)->View<Msg> {
// 	fn new(&self, value: &T) -> View<Msg> {
// 		unimplemented!()
// 	}
// }

// impl<Msg> Clone for Updater<Msg> {
// 	fn clone(&self) -> Self {
// 		Updater(self.0.clone())
// 	}
// }

// pub(crate) enum Linked<Msg> {
// 	New(NewLinked<Msg>),
// 	Dom(DomLinked<Msg>),
// }

// pub(crate) struct NewLinked<Msg> {
// 	pub(crate) segment: Vec<View<Msg>>,
// 	pub(crate) removed: Vec<View<Msg>>,
// }
// pub(crate) struct DomLinked<Msg> {
// 	pub(crate) segment: Vec<Dom<Msg>>,
// 	pub(crate) removed: Vec<Dom<Msg>>,
// }


