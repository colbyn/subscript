#[macro_use]
pub mod macros;
pub mod events;
pub mod attributes;
pub mod css;

use std::fmt;
use std::fmt::Debug;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use either::Either;
use serde::{self, Serialize, Deserialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;

use crate::browser::{self, Browser, Callback, console, DomRef};
use crate::ui::dom::style_mount::*;
use crate::ui::html::css::*;
use crate::ui::html::css::CssValue;
use crate::ui::html::events::*;
use crate::ui::html::attributes::*;
use crate::ui::dom;


///////////////////////////////////////////////////////////////////////////////
// CHILDREN
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Children<Msg> {
    value: Rc<RefCell<Vec<Html<Msg>>>>
}

impl<Msg: Clone + Debug + 'static> Children<Msg> {
    pub fn new() -> Self {
        Children {
            value: Rc::new(RefCell::new(Vec::new())),
        }
    }
    pub fn insert(&self, new: Html<Msg>) {
        self.value.borrow_mut().push(new);
    }
    pub fn clear(&self, style_mount: &StyleMount) {
        for child in self.value.borrow().iter() {
            child.clear(style_mount);
        }
        self.value.borrow_mut().clear();
    }
    pub fn init(&self, parent_node: &DomRef, style_mount: &StyleMount) {
        for child in self.value.borrow().iter() {
            child.init(style_mount);
            parent_node.append_child(child.dom_ref());
        }
    }
    pub fn sync(&self, other: &Children<Msg>, parent_node: &DomRef, style_mount: &StyleMount) {
        let equal_length = {
            self.value.borrow().len() == other.value.borrow().len()
        };
        if equal_length {
            for (c1, c2) in self.value.borrow().iter().zip(other.value.borrow().iter()) {
                c1.sync(c2, style_mount);
            }
        } else {
            // REMOVE
            self.clear(style_mount);
            self.value.replace(other.value.borrow().clone());
            // SET
            self.init(parent_node, style_mount);
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// NODE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Tag<Msg> {
    pub tag: String,
    pub id: String,
    pub dom_ref: DomRef,
    pub attributes: Attributes,
    pub styling: Styling,
    pub events: Events<Msg>,
    pub children: Children<Msg>,
}


impl<Msg: Clone + Debug + 'static> Tag<Msg> {
    pub fn new(tag: String) -> Self {
        let node_id = format!("_{}", rand::random::<u16>());
        Tag {
            tag: tag.clone(),
            id: node_id.clone(),
            dom_ref: DomRef::new(&tag),
            attributes: Attributes::new(),
            styling: Styling::new(node_id),
            events: Events::new(),
            children: Children::new(),
        }
    }
    pub fn init(&self, style_mount: &StyleMount) {
        // INIT CHILDREN
        self.children.init(&self.dom_ref, style_mount);
        // INIT SELF
        self.attributes.init(&self.dom_ref);
        self.styling.init(style_mount);
        self.events.init(&self.dom_ref);
        self.dom_ref.set_attribute("id", self.id.as_str());
    }
    pub fn clear(&self, style_mount: &StyleMount) {
        // CLEAR CHILDREN
        self.children.clear(style_mount);
        // CLEAR SELF
        self.events.clear(&self.dom_ref);
        self.styling.clear(style_mount);
    }
    pub fn sync(&self, other: &Tag<Msg>, style_mount: &StyleMount) {
        // SYNC CHILDREN
        self.children.sync(&other.children, &self.dom_ref, style_mount);
        // SYNC SELF
        self.attributes.sync(&other.attributes, &self.dom_ref);
        self.styling.sync(&other.styling, style_mount);
        self.events.sync(&other.events, &self.dom_ref);
    }
}




///////////////////////////////////////////////////////////////////////////////
// TEXT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Text {
    value: Rc<RefCell<String>>,
    dom_ref: DomRef,
}

impl Text {
    pub fn new(value: String) -> Self {
        Text {
            dom_ref: DomRef::new_text(value.as_str()),
            value: Rc::new(RefCell::new(value)),
        }
    }
    pub fn init(&self) {
        self.dom_ref.set_text_content(&self.value.borrow().as_str());
    }
    pub fn sync(&self, other: &Text) {
        let unchanged = self.value.borrow().as_str() == other.value.borrow().as_str();
        if !unchanged {
            self.value.replace(other.value.borrow().clone());
            self.init();
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// HTML
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum Html<Msg> {
    Tag(Tag<Msg>),
    Text(Text)
}


impl<Msg: Clone + Debug + 'static> Html<Msg> {
    pub fn new_node(tag: String) -> Self {
        Html::Tag(Tag::new(tag))
    }
    pub fn new_text(value: String) -> Self {
        Html::Text(Text::new(value))
    }
    pub fn get_node(&self) -> Option<&Tag<Msg>> {
        match &self {
            Html::Tag(node) => Some(node),
            _ => None
        }
    }
    pub fn get_text(&self) -> Option<&Text> {
        match &self {
            Html::Text(text) => Some(text),
            _ => None
        }
    }
    pub fn dom_ref(&self) -> &DomRef {
        match &self {
            Html::Tag(node) => &node.dom_ref,
            Html::Text(text) => &text.dom_ref,
        }
    }
    pub fn add_event_handler(&self, event_name: String, fun: Rc<Fn(JsValue)->Msg>) {
        if let Some(node) = self.get_node() {
            node.events.insert_event_handler(event_name, fun);
        }
    }
    pub fn add_attribute(&self, new: Attribute) {
        if let Some(node) = self.get_node() {
            node.attributes.add_attribute(new);
        }
    }
    pub fn add_style(&self, new: Style) {
        if let Some(node) = self.get_node() {
            node.styling.add_style(new);
        }
    }
    pub fn add_child(&self, new: Html<Msg>) {
        if let Some(node) = self.get_node() {
            node.children.insert(new);
        }
    }
    pub fn tick(&self) -> Vec<Msg> {
        let mut messages: Vec<Msg> = Vec::new();
        if let Some(node) = self.get_node() {
            for child in node.children.value.borrow().iter() {
                messages.append(&mut child.tick());
            }
            messages.append(&mut node.events.tick());
        }
        messages
    }
    pub fn init(&self, style_mount: &StyleMount) {
        match &self {
            Html::Tag(node) => node.init(style_mount),
            Html::Text(text) => text.init(),
        }
    }
    pub fn clear(&self, style_mount: &StyleMount) {
        match &self {
            Html::Tag(node) => node.clear(style_mount),
            Html::Text(text) => (),
        }
    }
    pub fn sync(&self, other: &Html<Msg>, style_mount: &StyleMount) {
        match (&self, &other) {
            (Html::Tag(n1), Html::Tag(n2)) => n1.sync(n2, style_mount),
            (Html::Text(t1), Html::Text(t2)) => t1.sync(t2),
            _ => ()
        }
    }
}




