use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::*;
use std::marker::Sized;
use either::Either;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::browser::*;
use crate::process::data::*;
use crate::tree::offline::data::*;


///////////////////////////////////////////////////////////////////////////////
// INTERFACE - CSS
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_CSS: GlobalCss = {
        GlobalCss {
            dom_ref: {
                let browser = Browser::new();
                let dom_ref = DomRef::new("style");
                browser.body.append_child(&dom_ref);
                dom_ref
            },
            offline_nodes: RefCell::new(BTreeMap::new()),
            online_nodes: RefCell::new(BTreeMap::new()),
        }
    };
}

pub type StyleNodesTree = BTreeMap<NodeId, StyleNode>;

#[derive(Clone)]
pub struct GlobalCss {
    pub dom_ref: DomRef,
    pub offline_nodes: RefCell<StyleNodesTree>,
    pub online_nodes: RefCell<StyleNodesTree>,
}



///////////////////////////////////////////////////////////////////////////////
// INTERFACE - COMPONENT
///////////////////////////////////////////////////////////////////////////////




///////////////////////////////////////////////////////////////////////////////
// INTERFACE - HTML
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum LiveHtml<Msg> {
    Component(LiveComponent),
    Text(LiveText),
    Node(LiveNode<Msg>),
}

#[derive(Clone)]
pub struct LiveComponent {
    pub process: Box<ProcessHandle>,
}

#[derive(Clone)]
pub struct LiveText {
    pub dom_ref: DomRef,
    pub value: RefCell<String>,
}

#[derive(Clone)]
pub struct LiveNode<Msg> {
    pub dom_ref: DomRef,
    pub node_id: NodeId,
    pub tag: String,
    pub attributes: RefCell<BTreeMap<String, Either<bool, String>>>,
    pub events: RefCell<BTreeMap<EventType, Callback<Msg>>>,
    pub styling: RefCell<StyleNode>,
    pub children: RefCell<Vec<LiveHtml<Msg>>>,
}

