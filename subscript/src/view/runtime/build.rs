use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::model::incremental::{IVecSub};
use crate::view::dsl::{self as dsl, Dsl, View, SubComponent};
use crate::view::shared::*;
use crate::view::dom::*;
use crate::view::runtime::common::*;


impl<Msg> View<Msg> {
	pub(crate) fn build_root(&self) -> Dom<Msg> {
		let build_root = |view: &View<Msg>| -> Dom<Msg> {
			let mut mount: Element<Msg> = Element {
				dom_ref: browser::window().document.create_element("div"),
				auto_listeners: Vec::new(),
				tag: String::from("div"),
				attributes: HashMap::new(),
				events: Vec::new(),
				children: Vec::new(),
			};
			mount.children.push(view.build(&ElementEnv {
				tag: mount.tag.as_str(),
				dom_ref: &mount.dom_ref,
				rightward: &RefCell::new(None),
			}));
			browser::window().document.body.append_child(&mount.dom_ref);
			Dom::Element(mount)
		};
		build_root(self)
	}
	pub(crate) fn build(&self, env: &ElementEnv) -> Dom<Msg> {
		let window = browser::window();
		match &self.0 {
			Dsl::Text(x) => {
				let value = match &x.0 {
					Value::Dynamic(cell) => cell.value.borrow().clone(),
					Value::Static(value) => value.clone(),
				};
				let dom_ref = window.document
					.create_text_node(value.as_str());
				insert_child(&dom_ref, env);
				Dom::Text(Text{value: x.0.clone(), dom_ref})
			}
			Dsl::Element(x) => {
				let tag = x.tag.clone();
				let dom_ref = browser::window().document.create_element(tag.as_str());
				let new_env = ElementEnv {
					tag: tag.as_str(),
					dom_ref: &dom_ref,
					rightward: &RefCell::new(None),
				};
				let auto_listeners = get_and_add_auto_listeners::<Msg>(&new_env);
				let DomSegment{attributes,events, children} = build_dom_segment(&new_env, ViewSegment {
					attributes: &x.attributes,
					events: &x.events,
					children: &x.children,
				});
				insert_child(&dom_ref, env);
				Dom::Element(Element {dom_ref,auto_listeners,tag,attributes,events,children})
			}
			Dsl::Component(x) => {
				let dom_ref = window.document
					.create_element("div");
				insert_child(&dom_ref, env);
				Dom::Component(LiveComponent{dom_ref, inner: x.clone()})
			}
			Dsl::Mixin(x) => {
				let DomSegment{attributes,events, children} = build_dom_segment(env, ViewSegment {
					attributes: &x.attributes,
					events: &x.events,
					children: &x.children,
				});
				Dom::Mixin(Mixin {attributes,events,children})
			}
			Dsl::Control(dsl::Control::Toggle{pred, value}) => {
				let mut dom: Option<Dom<Msg>> = None;
				if pred.value.borrow().clone() {
					dom = Some(value.build(env));
				}
				Dom::Control(Control::Toggle(Box::new(Toggle{
					pred: pred.clone(),
					template: value.clone(),
					dom: RefCell::new(dom),
				})))
			}
			Dsl::Control(dsl::Control::Linked(sub)) => {
				sub.build(|children| {
					let results = build_dom_segment(env, ViewSegment{
						attributes: &HashMap::new(),
						events: &Vec::new(),
						children: &children,
					});
					results.children
				});
				Dom::Control(Control::Linked(sub.clone()))
			}
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
// UTILS
///////////////////////////////////////////////////////////////////////////////

struct ViewSegment<'a, Msg> {
	attributes: &'a HashMap<String, Either<Value<String>, Value<bool>>>,
	events: &'a Vec<EventHandler<Msg>>,
	children: &'a Vec<View<Msg>>,
}

struct DomSegment<Msg> {
	attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
	events: Vec<LiveEventHandler<Msg>>,
	children: Vec<Dom<Msg>>,
}

fn build_dom_segment<'a, Msg>(env: &ElementEnv<'a>, view_segment: ViewSegment<Msg>) -> DomSegment<Msg> {
	// SETUP
	let ViewSegment{attributes,events,children} = view_segment;
	let mut dom_segment = DomSegment {
		attributes: HashMap::new(),
		events: Vec::new(),
		children: Vec::new(),
	};
	// ATTRIBUTES
	for (key, value) in attributes.iter() {
		set_attribute(&key, &value, env);
		dom_segment.attributes.insert(key.to_string(), value.clone());
	}
	// EVENTS
	for event in events.iter() {
		let backend_callback = QueueCallback::new(
			env.dom_ref,
			event.event_type().as_str(),
			CallbackSettings::default()
		);
		dom_segment.events.push(LiveEventHandler{
			backend_callback,
			frontend_callback: event.clone(),
		});
	}
	// CHILDREN
	for new_child in children.into_iter().rev() {
		let child = new_child.build(env);
		env.rightward.replace(child.get_before_dom_ref());
		dom_segment.children.insert(0, child);
	}
	// DONE
	dom_segment
}



pub(crate) fn get_and_add_auto_listeners<'a, Msg>(env: &ElementEnv<'a>) -> Vec<browser::VoidCallback> {
	let mut auto_listeners = Vec::new();
	let mut add_prevent_default = |event_type: &str| {
		let settings = CallbackSettings{prevent_default: true, ..CallbackSettings::default()};
		let callback = browser::VoidCallback::new(env.dom_ref, "submit", settings);
		auto_listeners.push(callback);
	};
	match env.tag {
		"form" => add_prevent_default("submit"),
		"a" => add_prevent_default("click"),
		_ => {}
	}
	auto_listeners
}




