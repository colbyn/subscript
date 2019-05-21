use either::{Either, Left, Right};
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
	// let live: ITree<LiveNode<Msg>, LiveLeaf> = ITree::from(view(), &DomTreeLogic::default());
	// match live {
	// 	ITree::Leaf{data} => {}
	// 	ITree::Node{data, ..} => {
	// 		window.document.body.append_child(data.dom_ref.as_ref());
	// 	}
	// }
}

