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
use insertion_types::map::{self, IMap};
use insertion_types::tree::{self, ITree};


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type NodeId = String;
pub type Html<Msg> = ITree<Node<Msg>, Leaf>;
pub type Svg<Msg> = ITree<Node<Msg>, Leaf>;



///////////////////////////////////////////////////////////////////////////////
// COMPONENTS
///////////////////////////////////////////////////////////////////////////////

pub trait Component {
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



///////////////////////////////////////////////////////////////////////////////
// HTML TREE
///////////////////////////////////////////////////////////////////////////////


pub enum Leaf {
    Text(String),
    Component(Box<Component>),
}

pub struct Node<Msg> {
    pub tag: String,
    pub attributes: IMap<String, attributes::Attribute>,
    pub events: IMap<events::EventType, events::EventHandler<Msg>>,
    pub styling: css_dsl::Stylesheet,
}


///////////////////////////////////////////////////////////////////////////////
// MIXINS
///////////////////////////////////////////////////////////////////////////////

pub struct Mixin<Msg> {
    pub attributes: IMap<String, attributes::Attribute>,
    pub events: IMap<events::EventType, events::EventHandler<Msg>>,
    pub styling: css_dsl::Stylesheet,
}



///////////////////////////////////////////////////////////////////////////////
// DOM TREE EXTENSIONS
///////////////////////////////////////////////////////////////////////////////

pub trait Viewable<Msg> {
    fn normalize(&self) -> Mixin<Msg>;
}


