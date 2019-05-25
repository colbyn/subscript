#![allow(dead_code, unused, unused_variables)]


pub mod attributes;
pub mod events;
#[macro_use]
pub mod macros;
pub mod components;

use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::*;
use std::marker::Sized;
use either::Either::{self, Left, Right};
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};

use crate::attributes::*;
use crate::events::*;
use ss_trees::tree::*;
use ss_trees::tree::map::*;
use ss_css_types::api::*;
pub use components::*;

///////////////////////////////////////////////////////////////////////////////
// VIEW TREE WRAPPER
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct View<Msg: PartialEq>(pub ITree<ViewNode<Msg>, ViewLeaf>);

impl<Msg: PartialEq> View<Msg> {
    pub fn new_tag(tag: &str) -> Self {
        let view_node = ViewNode {
            tag: String::from(tag),
            attributes: HashMap::new(),
            events: HashMap::new(),
            styling: Stylesheet::default(),
        };
        View(ITree::new(Left(view_node)))
    }
    pub fn new_text(value: &str) -> Self {
        let view_leaf = ViewLeaf::Text(String::from(value));
        View(ITree::new(Right(view_leaf)))
    }
    pub fn append(&mut self, entry: impl Viewable<Msg>) {
        let Mixin{attributes, events, children, styling} = entry.normalize();
        for child in children {
            self.0.add_child(child.0);
        }
        if let Some(node) = self.0.get_node_mut() {
            node.attributes.extend(attributes);
            node.events.extend(events);
            node.styling.union(styling);
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type NodeId = String;
pub type Html<Msg> = ITree<ViewNode<Msg>, ViewLeaf>;
pub type Svg<Msg> = ITree<ViewNode<Msg>, ViewLeaf>;


///////////////////////////////////////////////////////////////////////////////
// MIXINS
///////////////////////////////////////////////////////////////////////////////

pub struct Mixin<Msg: PartialEq> {
    attributes: HashMap<String, AttributeValue>,
    events: HashMap<events::EventType, events::EventHandler<Msg>>,
    children: Vec<View<Msg>>,
    styling: Stylesheet,
}

impl<Msg: PartialEq> Default for Mixin<Msg> {
    fn default() -> Self {
        Mixin {
            attributes: HashMap::new(),
            events: HashMap::new(),
            children: Vec::new(),
            styling: Stylesheet::default(),
        }
    }
}

impl<Msg: PartialEq> Mixin<Msg> {
    pub fn add_attribute(&mut self, key: &str, value: AttributeValue) {
        self.attributes.insert(key.to_string(), value);
    }
    pub fn add_event_handler(&mut self, handler: events::EventHandler<Msg>) {
        let event_name = handler.event_name();
        self.events.insert(event_name, handler);
    }
    pub fn add_child(&mut self, child: View<Msg>) {
        self.children.push(child);
    }
    pub fn add_children(&mut self, xs: Vec<View<Msg>>) {
        let mut xs = xs;
        self.children.append(&mut xs);
    }
    pub fn add_style(&mut self, x: Style) {
        self.styling.add_style(x);
    }
}


///////////////////////////////////////////////////////////////////////////////
// VIEW TREE PRIMITIVES & EXTENSION INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub trait Viewable<Msg: PartialEq> {
    fn normalize(self) -> Mixin<Msg>;
}



impl<Msg: PartialEq> Viewable<Msg> for &str {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_child(View::new_text(self));
        mixin
    }
}
impl<Msg: PartialEq> Viewable<Msg> for String {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_child(View::new_text(self.as_str()));
        mixin
    }
}
impl<Msg: PartialEq> Viewable<Msg> for View<Msg> {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_child(self);
        mixin
    }
}
impl<Msg: PartialEq> Viewable<Msg> for Vec<View<Msg>> {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_children(self);
        mixin
    }   
}
impl<Msg: PartialEq> Viewable<Msg> for (String, AttributeValue) {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_attribute(self.0.as_str(), self.1);
        mixin
    }
}
impl<Msg: PartialEq> Viewable<Msg> for (&str, AttributeValue) {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_attribute(self.0, self.1);
        mixin
    }
}
impl<Msg: PartialEq> Viewable<Msg> for events::EventHandler<Msg> {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.add_event_handler(self);
        mixin
    }
}
impl<Msg: PartialEq> Viewable<Msg> for Style {
    fn normalize(self) -> Mixin<Msg> {
        let mut mixin = Mixin::default();
        mixin.styling.add_style(self);
        mixin
    }
}


///////////////////////////////////////////////////////////////////////////////
// INTERNAL ITREE TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ViewLeaf {
    Text(String),
    Component(Box<ViewComponent>),
}

impl PartialEq for ViewLeaf {
    fn eq(&self, other: &ViewLeaf) -> bool {
        match (self, other) {
            (ViewLeaf::Text(x), ViewLeaf::Text(y)) => {x == y}
            (ViewLeaf::Component(x), ViewLeaf::Component(y)) => {x == y}
            _ => false
        }
    }
}

impl ViewLeaf {
    pub fn is_text(&self) -> bool {
        match self {
            ViewLeaf::Text(_) => true,
            _ => false,
        }
    }
    pub fn is_component(&self) -> bool {
        match self {
            ViewLeaf::Component(_) => true,
            _ => false,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct ViewNode<Msg: PartialEq> {
    pub tag: String,
    pub attributes: HashMap<String, AttributeValue>,
    pub events: HashMap<events::EventType, events::EventHandler<Msg>>,
    pub styling: Stylesheet,
}



