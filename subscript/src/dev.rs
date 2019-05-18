use either::{Either, Left, Right};
use ::web_utils::dom;
use ::web_utils::prelude::*;
use ::insertion_types::tree::*;
use ::web_utils::js::{self, console};
use ::dom_tree::html::*;
use ::dom_tree::html::attributes::*;
use ::dom_tree::html::events::*;

// use insertion_types::tree::*;
// use insertion_types::tree::map::*;


#[derive(Clone, Debug, PartialEq)]
pub enum Msg {
    NoOp
}

// pub fn view() -> Html<Msg> {
//     let mut root = ITree::new(Right(ViewNode::new("div")));
//     root.add_child({
//     	let mut h1 = ITree::new(Right(ViewNode::new("h1")));
//     	h1.add_child(ITree::new(Left(ViewLeaf::Text(String::from("Hello World")))));
//     	h1
//     });
//     root
// }


pub fn main() {
	// let window = dom::window();
	// let ref dom_tree_api = dom_tree_api();
	// let live: ITree<LiveNode<Msg>, LiveLeaf> = ITree::from(view(), dom_tree_api);
	// match live {
	// 	ITree::Leaf{data} => {}
	// 	ITree::Node{data, ..} => {
	// 		window.document.body.append_child(data.dom_ref.as_ref());
	// 	}
	// }
}

