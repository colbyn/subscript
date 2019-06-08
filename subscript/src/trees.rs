use std::marker::*;
use std::cell::*;
use std::rc::*;
use ss_web_utils::{dom, js, js::console, prelude::*};

pub enum Dom {
	Text(Text),
	Tag(Tag),
}

pub struct Text {
	dom_ref: dom::Text,
	value: String,
}
pub struct Tag {
	tag: String,
	dom_ref: dom::Tag,
	children: Vec<Dom>,
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
	fn new_text(value: &str) -> Self {
		let dom_ref = dom::Text::new(&value);
		let value = String::from(value);
		Dom::Text(Text{dom_ref, value})
	}
	fn new_tag(tag: &str) -> Self {
		let tag = String::from(tag);
		let dom_ref = dom::Tag::new(&tag);
		Dom::Tag(Tag{tag, dom_ref, children: Vec::new()})
	}
	fn push_child(&mut self, child: Dom) {
		if let Some(parent) = self.get_tag_mut() {
			parent.dom_ref.append_child(child.get_dom_ref());
			parent.children.push(child);
		}
	}
	fn insert_child(&mut self, ix: usize, child: Dom) {
		if let Some(parent) = self.get_tag_mut() {
			let before_dom_ref = match parent.children.get(ix) {
				Some(before) => Some(before.get_dom_ref()),
				None => None
			};
			if let Some(before_dom_ref) = before_dom_ref {
				parent.dom_ref.insert_before(child.get_dom_ref(), before_dom_ref);
				parent.children.insert(ix, child);
			}
			else {
				parent.dom_ref.append_child(child.get_dom_ref());
				parent.children.push(child);
			}
		}
	}
}

///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////



fn view() -> Dom {
	let mut root = Dom::new_tag("div");
	root.push_child({
		let mut h1 = Dom::new_tag("h1");
		h1.push_child(Dom::new_text("Hello World"));
		h1
	});
	root
}



pub fn run() {
	let view = view();
	dom::window().document.body.append_child(view.get_dom_ref());
	std::mem::forget(view);
}