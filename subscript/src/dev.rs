use std::cell::*;
use std::rc::*;
use either::{Either, Left, Right};
use wasm_bindgen::JsValue;
use ss_web_utils::dom;
use ss_web_utils::prelude::*;
use ss_web_utils::js::{self, console};
use ss_trees::tree::*;
use ss_trees::map::*;
use ss_view_tree::*;
use ss_view_tree::attributes::*;
use ss_view_tree::events::*;
use ss_dom_tree::*;


#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    NoOp
}



pub fn view() -> Html<Msg> {
    let mut root = ITree::new(Left(ViewNode::new("main")));
    root.add_child({
    	let mut h1 = ITree::new(Left(ViewNode::new("h1")));
    	h1.add_child(ITree::new(Right(ViewLeaf::Text(String::from("Hello World")))));
    	h1
    });
    root.add_child({
    	let mut entry = ITree::new(Left(ViewNode::new("section")));
    	entry.add_child({
            let mut header = ITree::new(Left(ViewNode::new("header")));
            // h1.add_child(ITree::new(Left(ViewLeaf::Text(String::from("Hello World")))));
            header
        });
    	entry
    });
    root
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
	let live: LiveTree<Msg> = STree::from(&api, &mount, view());
}
