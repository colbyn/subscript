#[macro_use]
pub mod macros;

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
use crate::tree::offline::data::*;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::offline::*;
use crate::process::online::*;

impl<Msg> HtmlBuild<Msg> {
    pub fn new_node(tag: &str) -> Self {
        HtmlBuild::Node(NodeBuild {
            tag: String::from(tag),
            attributes: BTreeMap::new(),
            events: BTreeMap::new(),
            styling: StyleNode {
                self_rules: Vec::new(),
                self_media_queries: Vec::new(),
                self_pseudo_selectors: Vec::new(),
            },
            children: Vec::new(),
        })
    }
    pub fn new_text(value: &str) -> Self {
        HtmlBuild::Text(TextBuild {
            value: String::from(value),
        })
    }
    pub fn new_component<S: Spec>(spec: S) -> Self {
        HtmlBuild::Component(Box::new(OfflineProcess::from_spec(spec)))
    }
    pub fn unpack_node(&self) -> Option<&NodeBuild<Msg>> {
        match self {
            HtmlBuild::Node(node) => Some(&node),
            _ => None
        }
    }
    pub fn unpack_node_mut(&mut self) -> Option<&mut NodeBuild<Msg>> {
        match self {
            HtmlBuild::Node(node) => Some(node),
            _ => None
        }
    }
    pub fn merge_style_node(&mut self, other: StyleNode) {
        if let Some(node) = self.unpack_node_mut() {
            node.styling.self_rules.append(
                &mut other.self_rules.clone()
            );
            node.styling.self_media_queries.append(
                &mut other.self_media_queries.clone()
            );
            node.styling.self_pseudo_selectors.append(
                &mut other.self_pseudo_selectors.clone()
            );
        }
    }
}

impl<Msg: Clone + Debug + 'static> HtmlBuild<Msg> {
    pub fn add_event(&mut self, event_name: &str, closure: Rc<Fn(JsValue)->Msg>) {
        if let Some(node) = self.unpack_node_mut() {
            let closure: Rc<Fn(JsValue)->Option<Msg>> = Rc::new(move |value| Some(closure.clone()(value)));
            node.events.insert(String::from(event_name), Callback::new(closure.clone()));
        }
    }
}

impl<Msg: Clone + Debug> HtmlBuild<Msg> {
    pub fn add_attribute(&mut self, key: &str, value: Either<bool, String>) {
        if let Some(node) = self.unpack_node_mut() {
            node.attributes.insert(String::from(key), value);
        }
    }
    pub fn add_style(&mut self, style: Style) {
        if let Some(node) = self.unpack_node_mut() {
            match style {
                Style::SelfRule(x) => {
                    node.styling.self_rules.push(x);
                }
                Style::SelfMediaQuery(x) => {
                    node.styling.self_media_queries.push(x);
                }
                Style::SelfPseudoSelector(x) => {
                    node.styling.self_pseudo_selectors.push(x);
                }
            }
        }
    }
    pub fn add_child(&mut self, child: HtmlBuild<Msg>) {
        if let Some(node) = self.unpack_node_mut() {
            node.children.push(child);
        }
    }
}

