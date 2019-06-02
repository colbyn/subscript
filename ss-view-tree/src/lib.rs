#![allow(dead_code, unused, unused_variables)]


pub mod attributes;
pub mod events;
#[macro_use]
pub mod macros;
pub mod components;
pub mod styling;

use core::default::Default;
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
use ss_trees::ext::map::*;
use ss_css_properties::data::Style;
use ss_web_utils::{js::console};
pub use components::*;
use crate::attributes::*;
use crate::events::*;
use crate::styling::*;


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub struct Env<'a, Msg> {
    attributes: &'a mut HashMap<String, AttributeValue>,
    events: &'a mut HashMap<events::EventType, EventHandler<Msg>>,
    styling: &'a mut Stylesheet,
    children: &'a mut Vec<View<Msg>>,
}


///////////////////////////////////////////////////////////////////////////////
// VIEW
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct View<Msg>(pub ITree<ViewNode<Msg>, ViewLeaf>);

impl<'a, Msg> View<Msg> {
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
    pub fn merge(&mut self, something: impl Viewable<Msg>) {
        if let Some(node) = self.0.get_node_mut() {
            let mut children = Vec::new();
            let env = Env {
                attributes: &mut node.attributes,
                events: &mut node.events,
                styling: &mut node.styling,
                children: &mut children,
            };
            something.extend(env);
            for child in children {
                self.0.add_child(child.0);
            }
        }
    }
}

pub trait Viewable<Msg> {
    fn extend<'a>(self, env: Env<'a, Msg>);
}

impl<Msg> Viewable<Msg> for () {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        // let warning = vec![
        //     "Something in your view isn’t returning anything",
        //     "(i.e. returning ‘()’); perhaps theres a mistake somewhere?"
        // ];
        // console::warn(warning.join(" "));
    }
}
impl<Msg> Viewable<Msg> for &str {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        env.children.push(View::new_text(self));
    }
}
impl<Msg> Viewable<Msg> for String {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        env.children.push(View::new_text(self.as_str()));
    }
}
impl<Msg> Viewable<Msg> for View<Msg> {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        env.children.push(self);
    }
}
impl<Msg> Viewable<Msg> for Vec<View<Msg>> {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        let mut this = self;
        env.children.append(&mut this);
    }   
}
impl<Msg> Viewable<Msg> for (String, AttributeValue) {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        env.attributes.insert(self.0, self.1);
    }
}
impl<Msg> Viewable<Msg> for (&str, AttributeValue) {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        env.attributes.insert(String::from(self.0), self.1);
    }
}

///////////////////////////////////////////////////////////////////////////////
// MIXINS
///////////////////////////////////////////////////////////////////////////////

/// Comes with the `mix!{}` macro.
/// Works with any `Viewable`.
#[derive(Debug, PartialEq)]
pub struct Mixin<Msg> {
    attributes: HashMap<String, AttributeValue>,
    events: HashMap<events::EventType, EventHandler<Msg>>,
    styling: Stylesheet,
    children: Vec<View<Msg>>,
}

impl<Msg> Mixin<Msg> {
    /// Alias for `Mixin::default()`.
    pub fn new() -> Self {Mixin::default()}
    pub fn merge(&mut self, something: impl Viewable<Msg>) {
        let env = Env {
            attributes: &mut self.attributes,
            events: &mut self.events,
            styling: &mut self.styling,
            children: &mut self.children,
        };
        something.extend(env);
    }
}
impl<Msg> Viewable<Msg> for Mixin<Msg> {
    fn extend<'a>(self, env: Env<'a, Msg>) {
        env.attributes.extend(self.attributes);
        env.events.extend(self.events);
        env.children.extend(self.children);
        env.styling.merge(self.styling);
    }
}
impl<Msg> Default for Mixin<Msg> {
    fn default() -> Self {
        let attributes = HashMap::new();
        let events = HashMap::new();
        let styling = Stylesheet::default();
        let children = Vec::new();
        Mixin {attributes, events, styling, children}
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


#[derive(Debug)]
pub struct ViewNode<Msg> {
    pub tag: String,
    pub attributes: HashMap<String, AttributeValue>,
    pub events: HashMap<events::EventType, EventHandler<Msg>>,
    pub styling: Stylesheet,
}

impl<'a, Msg> PartialEq for ViewNode<Msg> {
    fn eq(&self, other: &ViewNode<Msg>) -> bool {
        self.tag == other.tag &&
        self.attributes == other.attributes &&
        self.events == other.events &&
        self.styling == other.styling
    }
}





