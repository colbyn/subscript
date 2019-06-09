use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use ss_web_utils::{dom, js, js::console, js::{DomEventHandler, DomCallbackSettings}, prelude::*};

///////////////////////////////////////////////////////////////////////////////
// VIEW PROPERTIES
///////////////////////////////////////////////////////////////////////////////

pub struct StaticEvents(HashMap<String, DomEventHandler>);
pub struct DynamicEvents {
	remove: Cell<bool>,
	handler: HashMap<String, DomEventHandler>,
}

pub struct StaticProperties {}

// pub enum DynamicAttribute {
// 	ToggleAttribute
// }

// pub struct DynamicProperties {
// 	static_attributes: HashMap<String, String>,
// 	dynamic_attributes: HashMap<String, String>,
// }


///////////////////////////////////////////////////////////////////////////////
// LIVE DOM TREE
///////////////////////////////////////////////////////////////////////////////

pub enum Dom {
	Text(Text),
	Tag(Tag),
}
pub struct Text {
	dom_ref: dom::Text,
	observer: Option<Observer<String>>
}
pub struct Tag {
	tag: String,
	dom_ref: dom::Tag,
	children: Vec<Segment>,
}
pub enum Segment {
	Static(Vec<Dom>),
	Dynamic(VecObserverData),
}

pub enum Chunk {
	Static {
		children: Vec<Dom>,
		attributes: HashMap<String, Either<String, bool>>,
		listeners: HashMap<String, DomEventHandler>,
	},
	Toggle {
		pred: Cell<bool>,
		children: Vec<Dom>,
		attributes: HashMap<String, Either<String, bool>>,
		listeners: HashMap<String, DomEventHandler>,
	},
	Dynamic {
		pred: Cell<bool>,
		children: Vec<Dom>,
		attributes: HashMap<String, Either<String, bool>>,
		listeners: HashMap<String, DomEventHandler>,
	},
}


impl Dom {
	fn get_dom_ref(&self) -> &dom::DomRef {
		match self {
			Dom::Tag(x) => &x.dom_ref,
			Dom::Text(x) => &x.dom_ref,
		}
	}
	fn get_tag_mut(&mut self) -> Option<&mut Tag> {
		match self {
			Dom::Tag(x) => Some(x),
			_ => None
		}
	}
	fn new_static_text(txt: &str) -> Self {
		let dom_ref = dom::Text::new(&txt);
		Dom::Text(Text{dom_ref, observer: None})
	}
	fn new_dynamic_text(cell: &Cell<String>) -> Self {
		let dom_ref = dom::Text::new(&cell.value.borrow());
		let observer = Observer::from(cell);
		Dom::Text(Text{dom_ref, observer: Some(observer)})
	}
	fn new_tag(tag: &str) -> Self {
		let tag = String::from(tag);
		let dom_ref = dom::Tag::new(&tag);
		Dom::Tag(Tag{tag, dom_ref, children: Vec::new()})
	}
	fn push_static_children(&mut self, children: Vec<Dom>) {
		if let Some(parent) = self.get_tag_mut() {
			// APPEND VIA FRAGMENT
			let fragment = dom::DocumentFragment::new();
			for child in children.iter() {
				fragment.append_child(child.get_dom_ref());
			}
			parent.dom_ref.append_child(&fragment);
			// SAVE & DONE
			parent.children.push(Segment::Static(children));
		}
	}
	fn insert_static_children(&mut self, ix: usize, children: Vec<Dom>) {
		if let Some(parent) = self.get_tag_mut() {
			let before_dom_ref = {
				let mut result: Option<Either<&Dom, Rc<RefCell<Dom>>>> = None;
				if let Some(before_segment) = parent.children.get(ix) {
					match before_segment {
						Segment::Static(data) => {
							if let Some(before) = data.first() {
								result = Some(Left(before));
							}
						}
						Segment::Dynamic(data) => {
							for x in data.live.borrow().iter() {
								if result.is_none() && x.is_unchanged() {
									result = Some(Right(
										x.expect_unchanged_get_dom()
									));
								}
							}
						}
					}
				}
				result
			};
			if let Some(Left(before_node)) = before_dom_ref {
				// INSERT VIA DOCUMENT-FRAGMENT
				let fragment = dom::DocumentFragment::new();
				for child in children.iter() {
					fragment.append_child(child.get_dom_ref());
				}
				parent.dom_ref.insert_before(&fragment, before_node.get_dom_ref());
				// SAVE & DONE
				parent.children.insert(ix, Segment::Static(children));
			}
			else if let Some(Right(before_node)) = before_dom_ref {
				// INSERT VIA DOCUMENT-FRAGMENT
				let fragment = dom::DocumentFragment::new();
				for child in children.iter() {
					fragment.append_child(child.get_dom_ref());
				}
				parent.dom_ref.insert_before(&fragment, before_node.borrow().get_dom_ref());
				// SAVE & DONE
				parent.children.insert(ix, Segment::Static(children));
			}
			else {
				// INSERT VIA DOCUMENT-FRAGMENT
				let fragment = dom::DocumentFragment::new();
				for child in children.iter() {
					fragment.append_child(child.get_dom_ref());
				}
				parent.dom_ref.append_child(&fragment);
				// SAVE & DONE
				parent.children.push(Segment::Static(children));
			}
		}
	}
	fn push_dynamic_children<T>(&mut self, list_cell: &ListCell<T>, transformer: impl DomMapper<T> + 'static) {
		if let Some(parent) = self.get_tag_mut() {
			// SETUP
			let children: Vec<ChangeState> = list_cell.data
				.borrow()
				.iter()
				.map(|x| {
					let dom = Rc::new(RefCell::new(transformer.map(x)));
					ChangeState::Unchanged{dom}
				})
				.collect::<Vec<_>>();
			let children = Rc::new(RefCell::new(children));
			let shared = VecObserverData {
				live: children.clone(),
				deleted: Rc::new(RefCell::new(Vec::new())),
			};
			list_cell.observers.borrow_mut().push(VecObserverFrontend {
				mapper: Box::new(transformer),
				shared: shared.clone(),
			});
			// APPEND VIA FRAGMENT
			let fragment = dom::DocumentFragment::new();
			for child in children.borrow().iter() {
				fragment.append_child(child.get_dom().borrow().get_dom_ref());
			}
			parent.dom_ref.append_child(&fragment);
			// SAVE & DONE
			parent.children.push(Segment::Dynamic(shared.clone()));
		}
	}
	fn insert_dynamic_children<T>(&mut self, ix: usize, list_cell: &ListCell<T>, transformer: impl DomMapper<T> + 'static) {
		if let Some(parent) = self.get_tag_mut() {
			// SETUP
			let children: Vec<ChangeState> = list_cell.data
				.borrow()
				.iter()
				.map(|x| {
					let dom = Rc::new(RefCell::new(transformer.map(x)));
					ChangeState::New{dom}
				})
				.collect::<Vec<_>>();
			let children = Rc::new(RefCell::new(children));
			let shared = VecObserverData {
				live: children.clone(),
				deleted: Rc::new(RefCell::new(Vec::new())),
			};
			list_cell.observers.borrow_mut().push(VecObserverFrontend {
				mapper: Box::new(transformer),
				shared: shared.clone(),
			});
			let before_dom_ref = {
				let mut result: Option<Either<&Dom, Rc<RefCell<Dom>>>> = None;
				if let Some(before_segment) = parent.children.get(ix) {
					match before_segment {
						Segment::Static(data) => {
							if let Some(before) = data.first() {
								result = Some(Left(before));
							}
						}
						Segment::Dynamic(data) => {
							for x in data.live.borrow().iter() {
								if result.is_none() && x.is_unchanged() {
									result = Some(Right(
										x.expect_unchanged_get_dom()
									));
								}
							}
						}
					}
				}
				result
			};
			if let Some(Left(before_node)) = before_dom_ref {
				// INSERT VIA DOCUMENT-FRAGMENT
				let fragment = dom::DocumentFragment::new();
				for child in children.borrow().iter() {
					fragment.append_child(child.get_dom().borrow().get_dom_ref());
				}
				parent.dom_ref.insert_before(&fragment, before_node.get_dom_ref());
				// SAVE & DONE
				parent.children.push(Segment::Dynamic(shared.clone()));
			}
			else if let Some(Right(before_node)) = before_dom_ref {
				// INSERT VIA DOCUMENT-FRAGMENT
				let fragment = dom::DocumentFragment::new();
				for child in children.borrow().iter() {
					fragment.append_child(child.get_dom().borrow().get_dom_ref());
				}
				parent.dom_ref.insert_before(&fragment, before_node.borrow().get_dom_ref());
				// SAVE & DONE
				parent.children.push(Segment::Dynamic(shared.clone()));
			}
			else {
				// INSERT VIA DOCUMENT-FRAGMENT
				let fragment = dom::DocumentFragment::new();
				for child in children.borrow().iter() {
					fragment.append_child(child.get_dom().borrow().get_dom_ref());
				}
				parent.dom_ref.append_child(&fragment);
				// SAVE & DONE
				parent.children.push(Segment::Dynamic(shared.clone()));
			}
		}
	}
	fn tick(&mut self, parent: &dom::Tag) {
		match self {
			Dom::Text(text) => {
				if let Some(observer) = &text.observer {
					let updater = |new_text: &String| {
						text.dom_ref.set_text_content(new_text);
					};
					observer.sync(updater);
				}
			}
			Dom::Tag(tag) => {
				for segment in tag.children.iter_mut() {
					match segment {
						Segment::Static(segment) => {
							for child in segment {
								child.tick(&tag.dom_ref);
							}
						}
						Segment::Dynamic(segment) => {
							// DELETE
							for deleted in segment.deleted.borrow_mut().drain(..) {
								tag.dom_ref.remove_child(deleted.borrow().get_dom_ref());
							}
							// INSERT NEW & TICK VALUES
							let mut last: Option<Rc<RefCell<Dom>>> = None;
							for mut entry in segment.live.borrow_mut().iter_mut().rev() {
								match entry {
									ChangeState::New{dom} => {
										match last {
											Some(before) => {
												tag.dom_ref.insert_before(
													dom.borrow().get_dom_ref(),
													before.borrow().get_dom_ref()
												);
											}
											None => {
												tag.dom_ref.append_child(dom.borrow().get_dom_ref());
											}
										}
										dom.borrow_mut().tick(&tag.dom_ref);
										last = Some(dom.clone());
										*entry = ChangeState::Unchanged{dom: dom.clone()};
									}
									ChangeState::Unchanged{dom} => {
										dom.borrow_mut().tick(&tag.dom_ref);
										last = Some(dom.clone());
									}
								}
							}
							// POST-PROCESSING - SANITY CHECKS
							debug_assert!(segment.live.borrow().iter().all(|x| x.is_unchanged()));
						}
					}
				}
			}
		}
	}
}



///////////////////////////////////////////////////////////////////////////////
// CELLS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Cell<T> {
	value: Rc<RefCell<T>>,
}

impl<T> Cell<T> {
	fn new(value: T) -> Self {
		let value = Rc::new(RefCell::new(value));
		Cell {value}
	}
	fn set(&mut self, new: T) {
		self.value.replace(new);
	}
}
#[derive(Debug)]
pub struct Observer<T: Clone + ?Sized + PartialEq> {
	local: RefCell<T>,
	remote: Weak<RefCell<T>>,
}

impl<T: Clone + ?Sized + PartialEq> Observer<T> {
	fn from(cell: &Cell<T>) -> Self {
		let remote = Rc::downgrade(&cell.value);
		let local = RefCell::new(cell.value.borrow().clone());
		Observer {local, remote}
	}
	fn sync(&self, on_change: impl Fn(&T)) {
		if let Some(remote) = self.remote.upgrade() {
			let unchanged = *self.local.borrow() == *remote.borrow();
			if !unchanged {
				let new_value = remote.borrow().clone();
				on_change(&new_value);
				self.local.replace(new_value);
			}
		}
	}
}


///////////////////////////////////////////////////////////////////////////////
// LIST CELLS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct ListCell<T>{
	data: Rc<RefCell<Vec<T>>>,
	observers: Rc<RefCell<Vec<VecObserverFrontend<T>>>>,
}

impl<T> ListCell<T> {
	pub fn new() -> Self {
		let data = Rc::new(RefCell::new(Vec::new()));
		let observers = Rc::new(RefCell::new(Vec::new()));
		ListCell{data, observers}
	}
	pub fn push(&mut self, new: T) {
		for observer in self.observers.borrow_mut().iter_mut() {
			let new = ChangeState::New{
				dom: Rc::new(RefCell::new(observer.mapper.map(&new))),
			};
			observer.shared.live.borrow_mut().push(new);
		}
		self.data.borrow_mut().push(new);
	}
	pub fn insert(&mut self, ix: usize, new: T) {
		for observer in self.observers.borrow_mut().iter_mut() {
			let new = ChangeState::New{
				dom: Rc::new(RefCell::new(observer.mapper.map(&new))),
			};
			observer.shared.live.borrow_mut().insert(ix, new);
		}
		self.data.borrow_mut().insert(ix, new);
	}
	pub fn remove(&mut self, ix: usize) -> Option<T> {
		if ix < self.data.borrow().len() {
			for observer in self.observers.borrow_mut().iter_mut() {
				let removed = observer.shared.live.borrow_mut().remove(ix);
				observer.shared.deleted.borrow_mut().push(removed.into_inner());
			}
			Some(self.data.borrow_mut().remove(ix))
		} else {
			None
		}
	}
}

pub struct VecObserverFrontend<T> {
	mapper: Box<DomMapper<T>>,
	shared: VecObserverData,
}
#[derive(Clone)]
pub struct VecObserverData {
	live: Rc<RefCell<Vec<ChangeState>>>,
	deleted: Rc<RefCell<Vec<Rc<RefCell<Dom>>>>>,
}
#[derive(Clone)]
pub enum ChangeState {
	Unchanged {
		dom: Rc<RefCell<Dom>>,
	},
	New {
		dom: Rc<RefCell<Dom>>,
	}
}

impl ChangeState {
	fn into_inner(self) -> Rc<RefCell<Dom>> {
		match self {
			ChangeState::Unchanged{dom} => dom,
			ChangeState::New{dom} => dom,
		}
	}
	fn is_unchanged(&self) -> bool {
		match self {
			ChangeState::Unchanged{..} => true,
			_ => false
		}
	}
	fn get_dom(&self) -> Rc<RefCell<Dom>> {
		match self {
			ChangeState::Unchanged{dom} => dom.clone(),
			ChangeState::New{dom} => dom.clone(),
		}
	}
	fn expect_unchanged_get_dom(&self) -> Rc<RefCell<Dom>> {
		match self {
			ChangeState::Unchanged{dom} => {dom.clone()}
			_ => panic!()
		}
	}
}


pub trait DomMapper<T> {
	fn map(&self, new: &T)->Dom;
}
impl<T> DomMapper<T> for (Fn(&T)->Dom) {
	fn map(&self, new: &T)->Dom {
		self(new)
	}
}
impl<T> DomMapper<T> for Box<Fn(&T)->Dom> {
	fn map(&self, new: &T)->Dom {
		self(new)
	}
}



///////////////////////////////////////////////////////////////////////////////
// PROGRAM
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub(crate) static SYSTEM_TICK_CALLBACK: RefCell<Box<Fn()>> = {
    	RefCell::new(Box::new(|| {}))
    };
}

pub struct Program {
	mount: dom::Tag,
	view: RefCell<Dom>,
}

impl Program {
	fn tick(&self) {
		self.view.borrow_mut().tick(&self.mount);
	}
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct TodoEntry {
	title: String,
}

fn render_todo_entry(entry: &TodoEntry) -> Dom {
	let mut root = Dom::new_tag("div");
	root.push_static_children(vec![{
		let mut form = Dom::new_tag("form");
		form.push_static_children(vec![{
			let mut label = Dom::new_tag("label");
			label.push_static_children(vec![
				Dom::new_static_text(entry.title.as_str()),
			]);
			label
		}]);
		form
	}]);
	root
}


fn view() -> Dom {
	let mut root = Dom::new_tag("main");
	root.push_static_children(vec![{
		let mut h1 = Dom::new_tag("h1");
		h1.push_static_children(vec![Dom::new_static_text("Top")]);
		h1
	}]);
	let mut todo_cell = ListCell::<TodoEntry>::new();
	todo_cell.push(TodoEntry {
		title: String::from("test1")
	});
	todo_cell.push(TodoEntry {
		title: String::from("test2")
	});
	todo_cell.push(TodoEntry {
		title: String::from("test3")
	});
	let mapper: Box<Fn(&TodoEntry)->Dom> = Box::new(move |x| render_todo_entry(x));
	root.push_dynamic_children(&todo_cell, mapper);
	let cb = dom::window().set_timeout(3000, {
		let mut todo_cell = todo_cell.clone();
		move || {
			console::log("timeout");
			todo_cell.insert(1, TodoEntry {
				title: String::from("new")
			});
		}
	});
	std::mem::forget(todo_cell);
	std::mem::forget(cb);
	root
}

pub fn run() {
	let live = view();
	let progam = Program {
		mount: {
			let mount = dom::Tag::new("div");
			mount.append_child(live.get_dom_ref());
			mount.set_attribute("view-wrapper", "");
			dom::window().document.body.append_child(&mount);
			mount
		},
		view: RefCell::new(live),
	};
	SYSTEM_TICK_CALLBACK.with(move |runner| {
		runner.replace(Box::new(move || progam.tick()));
	});
}