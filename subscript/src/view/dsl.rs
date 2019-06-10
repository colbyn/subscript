use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::model::reactive::{Signal, SignalSub, Status};
use crate::model::incremental::{IVecSub, IVec};
use crate::program::spec::Spec;
use crate::view::shared::*;

///////////////////////////////////////////////////////////////////////////////
// VIEW
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct View<Msg>(pub(crate) Dsl<Msg>);

impl<Msg: 'static> View<Msg> {
	pub fn new_text(value: &str) -> Self {
		View(Dsl::Text(Text(Value::Static(String::from(value)))))
	}
	pub fn new_text_signal(cell: &Signal<String>) -> Self {
		View(Dsl::Text(Text(Value::Dynamic(cell.new_subscriber()))))
	}
	pub fn new_element(tag: &str) -> Self {
		View(Dsl::Element(Element {
			tag: String::from(tag),
			styling: Styling::default(),
			attributes: HashMap::new(),
			events: Vec::new(),
			children: Vec::new(),
		}))
	}
	pub fn new_mixin() -> Self {
		View(Dsl::Mixin(Mixin {
			styling: Styling::default(),
			attributes: HashMap::new(),
			events: Vec::new(),
			children: Vec::new(),
		}))
	}
	pub fn new_linked_control<T: 'static>(vec: &IVec<T, Msg>, provision: impl Fn(&T)->View<Msg> + 'static) -> Self {
		let sub = vec.new_subscriber(provision);
		View(Dsl::Control(Control::Linked(sub)))
	}
	pub fn new_toggle_control(pred: &Signal<bool>, value: View<Msg>) -> Self {
		View(Dsl::Control(Control::Toggle {
			pred: pred.new_subscriber(),
			value: Rc::new(value),
		}))
	}
	pub fn new_component<S: Spec + 'static >(name: &str, spec: S) -> Self {
		View(Dsl::Component(SubComponent(Rc::new(Component {
			name: String::from(name),
			spec,
		}))))
	}
	pub fn text(&mut self, value: &str) {
		self.push_child(View::new_text(value));
	}
	pub fn text_cell(&mut self, value: &Signal<String>) {
		self.push_child(View::new_text_signal(value));
	}
	pub fn tag(&mut self, tag: &str, inner: impl FnMut(&mut View<Msg>)) {
		if let Some(env) = self.get_env() {
			let mut new_element = View::new_element(tag);
			let mut inner = Box::new(inner);
			inner(&mut new_element);
			env.children.push(new_element);
		}
	}
	pub fn push_child(&mut self, child: View<Msg>) {
		if let Some(env) = self.get_env() {
			env.children.push(child);
		}
	}
	pub fn add_styling(&mut self, new: Styling) {
		if let Some(env) = self.get_env() {
			env.styling.extend(new);
		}
	}
	pub fn get_env<'a>(&'a mut self) -> Option<Env<'a, Msg>> {
		match &mut self.0 {
			Dsl::Element(element) => {
				Some(Env {
					styling: &mut element.styling,
					attributes: &mut element.attributes,
					events: &mut element.events,
					children: &mut element.children,
				})
			}
			Dsl::Mixin(mixin) => {
				Some(Env {
					styling: &mut mixin.styling,
					attributes: &mut mixin.attributes,
					events: &mut mixin.events,
					children: &mut mixin.children,
				})
			}
			Dsl::Control(Control::Toggle{pred, value}) => {
				unimplemented!()
			}
			Dsl::Control(Control::Linked(sub)) => {
				unimplemented!()
			}
			Dsl::Component(component) => None,
			Dsl::Text(text) => None,
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
// VIEW - INTERNAL
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub(crate) enum Dsl<Msg> {
	Component(SubComponent),
	Text(Text),
	Element(Element<Msg>),
	Mixin(Mixin<Msg>),
	Control(Control<Msg>),
}
#[derive(Clone)]
pub(crate) struct SubComponent(Rc<SubComponentImpl>);

#[derive(Debug)]
pub(crate) struct Text(pub Value<String>);

#[derive(Debug)]
pub(crate) struct Element<Msg> {
	pub(crate) tag: String,
	pub(crate) styling: Styling,
	pub(crate) attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
	pub(crate) events: Vec<EventHandler<Msg>>,
	pub(crate) children: Vec<View<Msg>>,
}

#[derive(Debug)]
pub(crate) struct Mixin<Msg> {
	pub(crate) styling: Styling,
	pub(crate) attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
	pub(crate) events: Vec<EventHandler<Msg>>,
	pub(crate) children: Vec<View<Msg>>,
}

#[derive(Debug)]
pub(crate) enum Control<Msg> {
	Linked(IVecSub<Msg>),
	Toggle {
		pred: SignalSub<bool>,
		value: Rc<View<Msg>>,
	},
}


impl std::fmt::Debug for SubComponent {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "SubComponent")
	}
}



pub struct Env<'a, Msg> {
	pub(crate) styling: &'a mut Styling,
    pub(crate) attributes: &'a mut HashMap<String, Either<Value<String>, Value<bool>>>,
    pub(crate) events: &'a mut Vec<EventHandler<Msg>>,
    pub(crate) children: &'a mut Vec<View<Msg>>,
}


