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


///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Attributes {
    attributes: Rc<RefCell<Vec<Attribute>>>
}

impl Attributes {
    pub fn new() -> Self {
        Attributes {
            attributes: Rc::new(RefCell::new(Vec::new()))
        }
    }
    pub fn add_attribute(&self, new: Attribute) {
        self.attributes.borrow_mut().push(new);
    }
    pub fn delete_attribute(&self, given_key: &String) {
        self.attributes.borrow_mut()
            .retain(|attr| &attr.key() != given_key);
    }
    pub fn init(&self, live: &DomRef) {
        for attr in self.attributes.borrow().iter() {
            match attr {
                Attribute::Pair{key, value} => {
                    live.set_attribute(
                        key.as_str(),
                        value.as_str(),
                    );
                }
                Attribute::Toggle{key, value} => {
                    if value.clone() {
                        live.set_attribute(key.as_str(),"");
                    }
                }
            }
        }
    }
    pub fn sync(&self, other: &Attributes, live: &DomRef) {
        let unchanged = self.attributes == other.attributes;
        if !unchanged {
            // CLEAR
            for attr in self.attributes.borrow().iter() {
                live.remove_attribute(attr.key().as_str());
            }
            self.attributes.borrow_mut().clear();
            // SET
            self.attributes.replace(other.attributes.borrow().clone());
            self.init(live);
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTE
///////////////////////////////////////////////////////////////////////////////


#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    Pair {
        key: String,
        value: String,
    },
    Toggle {
        key: String,
        value: bool,
    }
}

impl Attribute {
    pub fn stringify(&self) -> String {
        match self {
            Attribute::Pair{key, value} => format!("{k}={v}", k=key, v=value),
            Attribute::Toggle{key, ..} => format!("{k}", k=key),
        }
    }
    pub fn is_pair(&self) -> bool {
        match &self {
            Attribute::Pair{..} => true,
            _ => false,
        }
    }
    pub fn key(&self) -> String {
        match &self {
            Attribute::Pair{key, ..} => key.clone(),
            Attribute::Toggle{key, ..} => key.clone(),
        }
    }
    pub fn value(&self) -> String {
        match &self {
            Attribute::Pair{value, ..} => value.clone(),
            Attribute::Toggle{..} => String::new(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTE VALUES
///////////////////////////////////////////////////////////////////////////////

pub trait AttributeValue {
    fn stringify(&self) -> String;
}

impl AttributeValue for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}

impl AttributeValue for &str {
    fn stringify(&self) -> String {
        self.clone().to_owned()
    }
}


