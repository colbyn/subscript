#![allow(dead_code, unused, unused_variables)]


pub mod attributes;
pub mod events;
pub mod macros;
pub mod css;

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

use ss_trees::tree::*;
use ss_trees::map::*;
use ss_css_types::api::*;

///////////////////////////////////////////////////////////////////////////////
// VIEW TREE WRAPPER
///////////////////////////////////////////////////////////////////////////////

pub struct View<Msg: PartialEq>(pub(crate) ITree<ViewNode<Msg>, ViewLeaf>);

impl<Msg: PartialEq> View<Msg> {
    pub fn new_tag(tag: &str) -> Self {
        let view_node = ViewNode {
            tag: String::from(tag),
            attributes: IMap::new(),
            events: IMap::new(),
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
            node.attributes.union(attributes);
            node.events.union(events);
            node.styling.union(styling);
        }
    }
    pub fn add_style(&mut self, x: Style) {
        if let Some(node) = self.0.get_node_mut() {
            node.styling.add_style(x);
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
// COMPONENTS
///////////////////////////////////////////////////////////////////////////////

pub trait Component
{
    fn spec_type_id(&self) -> TypeId;
    fn box_clone(&self) -> Box<Component>;
    fn spawn(&self) -> Box<()>;
}

impl Clone for Box<Component>
{
    fn clone(&self) -> Box<Component> {
        self.box_clone()
    }
}
impl PartialEq for Component {
    fn eq(&self, other: &Component) -> bool {
        self.spec_type_id() == other.spec_type_id()
    }
}
impl Debug for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "QueueCallback")
    }
}

///////////////////////////////////////////////////////////////////////////////
// MIXINS
///////////////////////////////////////////////////////////////////////////////

pub struct Mixin<Msg: PartialEq> {
    attributes: IMap<String, attributes::Attribute>,
    events: IMap<events::EventType, events::EventHandler<Msg>>,
    children: Vec<View<Msg>>,
    styling: Stylesheet,
}

impl<Msg: PartialEq> Default for Mixin<Msg> {
    fn default() -> Self {
        Mixin {
            attributes: IMap::new(),
            events: IMap::new(),
            children: Vec::new(),
            styling: Stylesheet::default(),
        }
    }
}

impl<Msg: PartialEq> Mixin<Msg> {
    pub fn add_attribute(&mut self, key: &str, value: attributes::Attribute) {
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
// VIEW TREE EXTENSIONS
///////////////////////////////////////////////////////////////////////////////

pub trait Viewable<Msg: PartialEq> {
    fn normalize(self) -> Mixin<Msg>;
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
impl<Msg: PartialEq> Viewable<Msg> for (&str, attributes::Attribute) {
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


///////////////////////////////////////////////////////////////////////////////
// INTERNAL ITREE TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ViewLeaf {
    Text(String),
    Component(Box<Component>),
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


#[derive(PartialEq)]
#[derive(Debug)]
pub struct ViewNode<Msg: PartialEq> {
    pub tag: String,
    pub attributes: IMap<String, attributes::Attribute>,
    pub events: IMap<events::EventType, events::EventHandler<Msg>>,
    pub styling: Stylesheet,
}

impl<Msg: PartialEq> ViewNode<Msg> {
    pub fn new(tag: &str) -> Self {
        ViewNode {
            tag: String::from(tag),
            attributes: IMap::new(),
            events: IMap::new(),
            styling: Stylesheet::default(),
        }
    }
}


