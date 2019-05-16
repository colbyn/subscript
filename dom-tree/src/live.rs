use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;
use std::any::*;
use std::collections::*;
use either::Either::{self, Left, Right};

use web_utils::dom;
use web_utils::js::{self, console, EventCallback};
use insertion_types::map::*;
use insertion_types::tree::*;

use crate::html::*;

///////////////////////////////////////////////////////////////////////////////
// LIVE DOM-REF
///////////////////////////////////////////////////////////////////////////////

pub enum DomRef {
    Text(dom::Text),
    Tag(dom::Tag),
}

///////////////////////////////////////////////////////////////////////////////
// LIVE COMPONENT
///////////////////////////////////////////////////////////////////////////////

pub type ProcessId = String;

pub trait LiveComponent {
    fn spec_type_id(&self) -> TypeId;
    fn process_id(&self) -> ProcessId;
    fn dom_ref(&self) -> &DomRef;
    fn tick(&self, sub_enqueue: &Vec<Rc<Any>>);
    fn box_clone(&self) -> Box<LiveComponent>;
}

impl Clone for Box<LiveComponent> {
    fn clone(&self) -> Box<LiveComponent> {
        self.box_clone()
    }
}

///////////////////////////////////////////////////////////////////////////////
// LIVE DOM TREE
///////////////////////////////////////////////////////////////////////////////


pub enum LiveLeaf {
    Text {
        dom_ref: Box<dom::Text>,
        value: String,
    },
    Component {
        dom_ref: Box<dom::Tag>,
        value: Box<Component>,
    },
}

pub struct LiveNode<Msg> {
    pub dom_ref: Box<dom::Tag>,
    pub tag: String,
    pub attributes: IMap<String, attributes::Attribute>,
    pub events: IMap<events::EventType, events::EventHandler<Msg>>,
    pub styling: css_dsl::Stylesheet,
}



