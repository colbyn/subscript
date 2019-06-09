use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use crate::view::{dsl::View, dom::Dom};
use crate::view::extras::{DomThunk, EvalDomThunk};


///////////////////////////////////////////////////////////////////////////////
// INTERNAL MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub struct ToView<T, Msg>(Box<Fn(&T)->View<Msg>>);

impl<T, Msg> std::fmt::Debug for ToView<T, Msg> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "ToView")
	}
}


///////////////////////////////////////////////////////////////////////////////
// IVEC
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct IVec<T, Msg>{
	data: Rc<RefCell<Vec<T>>>,
	subscribers: Rc<RefCell<Vec<IVecSubDriver<T, Msg>>>>,
}

impl<T, Msg> Clone for IVec<T, Msg> {
	fn clone(&self) -> Self {
		let data = self.data.clone();
		let subscribers = self.subscribers.clone();
		IVec{data, subscribers}
	}
}

///////////////////////////////////////////////////////////////////////////////
// IVEC API
///////////////////////////////////////////////////////////////////////////////

impl<T, Msg> IVec<T, Msg> {
	pub fn new() -> Self {
		let subscribers = Rc::new(RefCell::new(Vec::new()));
		let data = Rc::new(RefCell::new(Vec::new()));
		IVec {data,subscribers}
	}
	pub fn push(&mut self, new: T) {
		for sub in self.subscribers.borrow().iter() {
			sub.push(&new);
		}
		self.data.borrow_mut().push(new);
	}
	pub fn insert(&mut self, ix: usize, new: T) {
		for sub in self.subscribers.borrow().iter() {
			sub.insert(ix, &new);
		}
		self.data.borrow_mut().insert(ix, new);	
	}
}

///////////////////////////////////////////////////////////////////////////////
// IVEC INTERNAL API
///////////////////////////////////////////////////////////////////////////////
impl<T, Msg> IVec<T, Msg> {
	pub(crate) fn new_subscriber(&self, init_view: impl Fn(&T)->View<Msg> + 'static) -> IVecSub<Msg> {
		let segment: Vec<View<Msg>> = self.data
			.borrow()
			.iter()
			.map(|x| init_view(x))
			.collect();
		let thunk = Rc::new(RefCell::new(IVecThunk::View(segment)));
		let sub_driver = IVecSubDriver {
			provision: ToView(Box::new(init_view)),
			thunk: Rc::downgrade(&thunk),
		};
		self.subscribers.borrow_mut().push(sub_driver);
		IVecSub(thunk)
	}
}


///////////////////////////////////////////////////////////////////////////////
// SUBSCRIPTION
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct IVecSubDriver<T, Msg> {
	provision: ToView<T, Msg>,
	thunk: Weak<RefCell<IVecThunk<Msg>>>,
}
#[derive(Debug)]
pub(crate) struct IVecSub<Msg>(Rc<RefCell<IVecThunk<Msg>>>);

// SHARED STATE
#[derive(Debug)]
enum IVecThunk<Msg> {
	Borrowed,
	View(Vec<View<Msg>>),
	Dom(IVecDom<Msg>),
}
#[derive(Debug)]
pub(crate) struct IVecDom<Msg> {
	pub active: Vec<DomThunk<Msg>>,
	pub removed: Vec<Dom<Msg>>,
}

impl<Msg> Clone for IVecSub<Msg> {
	fn clone(&self) -> Self {
		IVecSub(self.0.clone())
	}
}


///////////////////////////////////////////////////////////////////////////////
// SUBSCRIPTION IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl<T, Msg> IVecSubDriver<T, Msg> {
	fn if_view(&self, f: impl Fn(&mut Vec<View<Msg>>)) {
		match self.thunk.upgrade() {
			Some(thunk) => {
				match thunk.replace(IVecThunk::Borrowed) {
					IVecThunk::View(mut view) => {
						f(&mut view);
						thunk.replace(IVecThunk::View(view));
					}
					x @ IVecThunk::Dom(_) => {
						thunk.replace(x);
					}
					IVecThunk::Borrowed => {panic!()}
				}
			}
			None => ()
		}
	}
	fn if_dom(&self, f: impl Fn(&mut IVecDom<Msg>)) {
		match self.thunk.upgrade() {
			Some(thunk) => {
				match thunk.replace(IVecThunk::Borrowed) {
					x @ IVecThunk::View(_) => {
						thunk.replace(x);
					}
					IVecThunk::Dom(mut x) => {
						f(&mut x);
						thunk.replace(IVecThunk::Dom(x));
					}
					IVecThunk::Borrowed => {panic!()}
				}
			}
			None => ()
		}
	}
	pub(crate) fn push(&self, value: &T) {
		self.if_view(|view| {
			let new_view = (self.provision.0)(value);
			view.push(new_view);
		});
		self.if_dom(|dom| {
			let new_view = (self.provision.0)(value);
			dom.active.push(DomThunk::from_view(new_view));
		});
	}
	pub(crate) fn insert(&self, ix: usize, value: &T) {
		self.if_view(|view| {
			let new_view = (self.provision.0)(value);
			view.insert(ix, new_view);
		});
		self.if_dom(|dom| {
			let new_view = (self.provision.0)(value);
			dom.active.insert(ix, DomThunk::from_view(new_view));
		});
	}
	pub(crate) fn delete(&self, ix: usize) {
		self.if_view(|view| {
			view.remove(ix);
		});
		self.if_dom(|dom| {
			match dom.active.remove(ix).into_inner() {
				Right(x) => {
					dom.removed.push(x);
				}
				_ => {}
			}
		});
	}
}


///////////////////////////////////////////////////////////////////////////////
// VIEW/DOM API
///////////////////////////////////////////////////////////////////////////////

// ARGUMENT HELPER TYPE
pub(crate) struct IVecSync<Active, Removed> {
	pub active: Active,
	pub remove: Removed,
}

impl<Msg> IVecSub<Msg> {
	pub(crate) fn terminate(self) -> Either<Vec<View<Msg>>, IVecDom<Msg>> {
		match Rc::try_unwrap(self.0) {
			Ok(val) => {
				match val.into_inner() {
					IVecThunk::View(x) => Left(x),
					IVecThunk::Dom(x) => Right(x),
					IVecThunk::Borrowed => panic!(),
				}
			}
			Err(_) => panic!()
		}
	}
	pub(crate) fn inspect_dom(&self, f: &mut FnMut(&Dom<Msg>)) {
		// let mut f = Box::new(f);
		let inner: &IVecThunk<Msg> = &self.0.borrow();
		match inner {
			IVecThunk::Dom(dom) => {
				for entry in dom.active.iter() {
					entry.inspect(f);
				}
			}
			IVecThunk::View(_) => panic!(),
			IVecThunk::Borrowed => panic!(),
		}
	}
	pub(crate) fn build(&self, f: impl Fn(Vec<View<Msg>>)->Vec<Dom<Msg>>) {
		match self.0.replace(IVecThunk::Borrowed) {
			IVecThunk::View(view) => {
				let active = f(view)
					.into_iter()
					.map(|view| DomThunk::new(view))
					.collect();
				self.0.replace(IVecThunk::Dom(IVecDom {
					active,
					removed: Vec::new(),
				}));
			}
			IVecThunk::Dom{..} => panic!(),
			IVecThunk::Borrowed => panic!(),
		}
	}
	pub(crate) fn sync(&self, f: IVecSync<impl Fn(&mut Vec<DomThunk<Msg>>), impl Fn(Dom<Msg>)>) {
		match self.0.replace(IVecThunk::Borrowed) {
			IVecThunk::Dom(IVecDom {mut active, mut removed}) => {
				for node in removed.drain(..) {
					(f.remove)(node);
				}
				(f.active)(&mut active);
				self.0.replace(IVecThunk::Dom(IVecDom{
					active,
					removed,
				}));
			}
			IVecThunk::View(view) => panic!(),
			IVecThunk::Borrowed => panic!(),
		}	
	}
}



