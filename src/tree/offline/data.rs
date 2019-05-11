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

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::offline::*;
use crate::process::online::*;


///////////////////////////////////////////////////////////////////////////////
// BASICS - MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type EventType = String;
pub type NodeId = String;



///////////////////////////////////////////////////////////////////////////////
// BASICS - ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////

pub trait AttributeValue {
    fn normalize(&self) -> Either<bool, String>;
}

impl AttributeValue for String {
    fn normalize(&self) -> Either<bool, String> {
        Either::Right(self.clone())
    }
}

impl AttributeValue for &str {
    fn normalize(&self) -> Either<bool, String> {
        Either::Right(String::from(self.clone()))
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASICS - EVENTS
///////////////////////////////////////////////////////////////////////////////

pub trait EventValue<Msg> {
    fn normalize(&self) -> Callback<Msg>;
}

impl<Msg: Clone + Debug + 'static> EventValue<Msg> for Rc<Fn(JsValue)->Msg> {
    fn normalize(&self) -> Callback<Msg> {
        Callback::new(Rc::new({
            let this = self.clone();
            move |value| {
                let this = this.clone();
                Some(this.as_ref()(value))
            }
        }))
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASICS - STYLING
///////////////////////////////////////////////////////////////////////////////

pub trait CssValue {
    fn normalize(&self) -> String;
}

impl CssValue for String {
    fn normalize(&self) -> String {
        self.clone()
    }
}

impl CssValue for &str {
    fn normalize(&self) -> String {
        self.clone().to_owned()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct StyleNode {
    pub self_rules: Vec<Rule>,
    pub self_media_queries: Vec<SelfMediaQueryDeclaration>,
    pub self_pseudo_selectors: Vec<SelfPseudoDeclaration>,
}

impl StyleNode {
    pub fn new() -> Self {
        StyleNode {
            self_rules: Vec::new(),
            self_media_queries: Vec::new(),
            self_pseudo_selectors: Vec::new(),
        }
    }
}


#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
pub struct Rule {
    pub property: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct SelfMediaQueryDeclaration {
    pub selector: BTreeSet<Rule>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct SelfPseudoDeclaration {
    pub selector: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Style {
    SelfRule(Rule),
    SelfMediaQuery(SelfMediaQueryDeclaration),
    SelfPseudoSelector(SelfPseudoDeclaration),
}


///////////////////////////////////////////////////////////////////////////////
// HTML BUILDER
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum HtmlBuild<Msg> {
    Component(Box<OfflineProcessApi>),
    Text(TextBuild),
    Node(NodeBuild<Msg>)
}

#[derive(Clone)]
pub struct TextBuild {
    pub value: String
}

#[derive(Clone)]
pub struct NodeBuild<Msg> {
    pub tag: String,
    pub attributes: BTreeMap<String, Either<bool, String>>,
    pub events: BTreeMap<EventType, Callback<Msg>>,
    pub styling: StyleNode,
    pub children: Vec<HtmlBuild<Msg>>,
}


///////////////////////////////////////////////////////////////////////////////
// HTML MIXINS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Mixin<Msg> {
    pub attributes: BTreeMap<String, Either<bool, String>>,
    pub events: BTreeMap<EventType, Callback<Msg>>,
    pub styling: StyleNode,
    pub nodes: Vec<HtmlBuild<Msg>>,
}

