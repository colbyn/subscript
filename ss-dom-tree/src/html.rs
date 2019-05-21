pub mod attributes;
pub mod events;

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

use web_utils::dom;
use web_utils::js::{self, console, EventCallback};
use ss_trees::tree::*;
use ss_trees::map::*;


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type NodeId = String;
pub type Html<Msg> = ViewTree<Msg>;
pub type Svg<Msg> = ViewTree<Msg>;



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



///////////////////////////////////////////////////////////////////////////////
// HTML TREE
///////////////////////////////////////////////////////////////////////////////

pub type ViewTree<Msg> = ITree<ViewNode<Msg>, ViewLeaf>;

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
pub struct ViewNode<Msg: PartialEq> {
    pub tag: String,
    pub attributes: IMap<String, attributes::Attribute>,
    pub events: IMap<events::EventType, events::EventHandler<Msg>>,
}

impl<Msg: PartialEq> ViewNode<Msg> {
    pub fn new(tag: &str) -> Self {
        ViewNode {
            tag: String::from(tag),
            attributes: IMap::new(),
            events: IMap::new(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// MIXINS
///////////////////////////////////////////////////////////////////////////////

pub struct Mixin<Msg: PartialEq> {
    pub attributes: IMap<String, attributes::Attribute>,
    pub events: IMap<events::EventType, events::EventHandler<Msg>>,
}



///////////////////////////////////////////////////////////////////////////////
// DOM TREE EXTENSIONS
///////////////////////////////////////////////////////////////////////////////

pub trait Viewable<Msg: PartialEq> {
    fn normalize(&self) -> Mixin<Msg>;
}


