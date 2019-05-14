pub mod program;
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
use either::Either;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::JsValue;

use crate::browser::*;



///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type NodeId = String;
pub type Html<Msg> = Node<Msg>;
pub type Svg<Msg> = Node<Msg>;


///////////////////////////////////////////////////////////////////////////////
// INTERNAL SYNTAX RENDERING
///////////////////////////////////////////////////////////////////////////////

pub trait Stringify {
    fn stringify(&self) -> &str;
}

impl Stringify for &str {
    fn stringify(&self) -> &str {
        &self
    }
}


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
// CSS
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
// ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////

pub trait AttributeValue {
    fn normalize(&self) -> Either<bool, String>;
}

impl AttributeValue for String {
    fn normalize(&self) -> Either<bool, String> {
        Either::Right(self.clone())
    }
}

impl AttributeValue for &String {
    fn normalize(&self) -> Either<bool, String> {
        Either::Right(self.to_string())
    }
}

impl AttributeValue for &str {
    fn normalize(&self) -> Either<bool, String> {
        Either::Right(String::from(self.clone()))
    }
}

impl AttributeValue for u32 {
    fn normalize(&self) -> Either<bool, String> {
        Either::Right(format!(
            "{}", self.clone()
        ))
    }
}

impl AttributeValue for bool {
    fn normalize(&self) -> Either<bool, String> {
        Either::Left(self.clone())
    }
}


///////////////////////////////////////////////////////////////////////////////
// EVENTS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum EventType {
    OnClick,
    OnDoubleClick,
    OnMouseDown,
    OnMouseUp,
    OnMouseEnter,
    OnMouseLeave,
    OnMouseOver,
    OnMouseOut,
    OnInput,
    OnCheck,
    OnSubmit,
    OnBlur,
    OnFocus,
}

pub trait Event<Msg> {
    fn handler(&self, event: JsValue) -> Msg;
}


///////////////////////////////////////////////////////////////////////////////
// DOM TREE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum Node<Msg> {
    Component(Box<Component>),
    Text(Text),
    Tag(Tag<Msg>)
}

#[derive(Clone)]
pub struct Text {
    pub value: String
}

#[derive(Clone)]
pub struct Tag<Msg> {
    pub tag: String,
    pub attributes: BTreeMap<String, Either<bool, String>>,
    pub events: BTreeMap<EventType, Callback<Msg>>,
    pub styling: StyleNode,
    pub children: Vec<Node<Msg>>,
}


///////////////////////////////////////////////////////////////////////////////
// MIXINS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Mixin<Msg> {
    pub attributes: BTreeMap<String, Either<bool, String>>,
    pub events: BTreeMap<EventType, Callback<Msg>>,
    pub styling: StyleNode,
    pub nodes: Vec<Node<Msg>>,
}



///////////////////////////////////////////////////////////////////////////////
// DOM TREE EXTENSIONS
///////////////////////////////////////////////////////////////////////////////

pub trait Viewable<Msg> {
    fn normalize(&self) -> Mixin<Msg>;
}






