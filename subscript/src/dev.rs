use std::cell::*;
use std::rc::*;
use either::{Either, Left, Right};
use wasm_bindgen::JsValue;
use ss_web_utils::dom;
use ss_web_utils::prelude::*;
use ss_web_utils::js::{self, console};
use ss_trees::tree::*;
use ss_view_tree::*;
use ss_view_tree::attributes::*;
use ss_view_tree::events::*;
use ss_dom_tree::*;
use ss_css_types::internal::*;


#[derive(Debug, PartialEq, Clone)]
enum Msg {
    NoOp,
}

pub struct Model {

}


fn view() -> View<Msg> {view!{
	
}}

#[derive(Debug, PartialEq)]
pub struct AppSpec {
	value: u32
}

pub fn main() {
	let window = dom::window();
	let api = DomTreeLogic::default();
	let mount = Meta::Tag {
		dom_ref: {
			let mount = window.document.create_element("div");
			window.document.body.append_child(&mount);
			Rc::new(mount)
		},
	};
	let live: LiveTree<Msg> = STree::from(&api, &mount, view().0);
}
