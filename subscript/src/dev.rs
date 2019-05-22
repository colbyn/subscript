use std::cell::*;
use std::rc::*;
use either::{Either, Left, Right};
use wasm_bindgen::JsValue;
use ::web_utils::dom;
use ::web_utils::prelude::*;
use ::web_utils::js::{self, console};
use ss_trees::tree::*;
use ss_trees::map::*;
use ss_dom_tree::html::*;
use ss_dom_tree::html::attributes::*;
use ss_dom_tree::html::events::*;
use ss_dom_tree::live::*;


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

pub fn view1() -> Html<Msg> {
    let mut root = ITree::new(Left(ViewNode::new("main")));
    root.add_child({
        let mut h1 = ITree::new(Left(ViewNode::new("h1")));
        h1.add_child(ITree::new(Right(ViewLeaf::Text(String::from("Hello World")))));
        h1
    });
    root.add_child({
        let mut h1 = ITree::new(Left(ViewNode::new("h2")));
        h1.add_child(ITree::new(Right(ViewLeaf::Text(String::from("Hello World0")))));
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
    pub fn empty_initial_view() -> Html<Msg> {
        let mut root = ITree::new(Left(ViewNode::new("div")));
        root
    }
	let live: RefCell<LiveTree<Msg>> = RefCell::new(STree::from(&api, &mount, empty_initial_view()));
	live.borrow_mut().sync(&api, &mount, view());
    
    let cb = js::VoidCallback::new(Box::new(move |_| {
        console::log("timeout");
        live.borrow_mut().sync(&api, &mount, view1());
    }));
    window.set_timeout(&cb, 3000);
    std::mem::forget(cb);
}
