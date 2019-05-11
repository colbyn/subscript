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
use crate::tree::online::data::*;

use crate::process::app::*;
use crate::process::basics::*;
use crate::process::offline::*;
use crate::process::online::*;



pub fn unchanged<Msg: Clone>(x: &LiveHtml<Msg>, y: &HtmlBuild<Msg>) -> bool {
    match (x, y) {
        (LiveHtml::Node(x), HtmlBuild::Node(y)) => {
            let eq_tag = x.tag == y.tag;
            let eq_attributes: bool = {
                let ats: &BTreeMap<String, Either<bool, String>> =
                    &x.attributes.borrow();
                ats == &y.attributes
            };
            let eq_styling: bool = {
                let ss: &StyleNode = &x.styling.borrow();
                ss == &y.styling
            };
            let eq_events: bool = {
                let es: &BTreeMap<EventType, Callback<Msg>> =
                    &x.events.borrow();
                es == &y.events
            };
            let eq_children = {
                let xs: &Vec<LiveHtml<Msg>> = &x.children.borrow();
                if xs.len() == y.children.len() {
                    xs  .iter()
                        .zip(y.children.iter())
                        .all(|(x, y)| {
                            unchanged(x, y)
                        })
                } else {
                    false
                }
            };
            eq_tag && eq_styling && eq_attributes && eq_events && eq_children
        }
        (LiveHtml::Text(x), HtmlBuild::Text(y)) => {
            x.value.borrow().as_str() == y.value.as_str()
        }
        (LiveHtml::Component(x), HtmlBuild::Component(y)) => {
            x.process.spec_type_id() == y.spec_type_id()
        }
        _ => {false}
    }
}

pub fn sync_events<Msg: Clone>(
    xs: &RefCell<BTreeMap<EventType, Callback<Msg>>>,
    ys: &BTreeMap<EventType, Callback<Msg>>,
    dom_ref: &DomRef
)
{
    let zipable = {
        let xs: &BTreeMap<String, Callback<Msg>> = &xs.borrow();
        xs.keys().collect::<Vec<&String>>() == ys.keys().collect::<Vec<&String>>()
    };
    if zipable {
        xs  .borrow_mut()
            .iter_mut()
            .zip(ys.clone().iter_mut())
            .for_each(|(x, y)| {
                
            });
    } else {
        for (old_key, old_value) in xs.borrow().iter() {
            dom_ref.remove_event_listener(old_key, &old_value.js_function);
        }
        for (new_key, new_value) in ys.iter() {
            dom_ref.add_event_listener(new_key, &new_value.js_function);
        }
        xs.replace(ys.clone());
    }
}

pub fn sync_stateful_attribute_dom_values(
    tag: &str,
    dom_ref: &DomRef,
    key: &str,
    v1: &Either<bool, String>,
    v2: &Either<bool, String>
) {
    match (tag, key, v1, v2) {
        ("input", "value", Either::Right(old), Either::Right(new)) if old != new => {
            set_input_value(dom_ref, new);
        },
        ("input", "value", _, Either::Right(new)) => {
            set_input_value(dom_ref, new);
        },
        _ => {},
    }
}

pub fn sync_attribute(
    x: (&String, &mut Either<bool, String>),
    y: (&String, &mut Either<bool, String>),
    dom_ref: &DomRef,
    tag: &str,
) {
    let unchanged = {
        x.0 == y.0 &&
        x.1 == y.1
    };
    if !unchanged {
        *x.1 = y.1.clone();
        match &x.1 {
            Either::Left(true) => dom_ref.set_attribute(&x.0, ""),
            Either::Left(false) => dom_ref.remove_attribute(&x.0),
            Either::Right(value) => dom_ref.set_attribute(&x.0, value),
        }
        sync_stateful_attribute_dom_values(
            tag,
            dom_ref,
            x.0,
            x.1,
            y.1
        );
    }
}

pub fn sync_attributes(
    xs: &RefCell<BTreeMap<String, Either<bool, String>>>,
    ys: &BTreeMap<String, Either<bool, String>>,
    dom_ref: &DomRef,
    tag: &str,
) {
    let zipable = {
        let xs: &BTreeMap<String, Either<bool, String>> = &xs.borrow();
        xs.keys().collect::<Vec<&String>>() == ys.keys().collect::<Vec<&String>>()
    };
    if zipable {
        xs  .borrow_mut()
            .iter_mut()
            .zip(ys.clone().iter_mut())
            .for_each(|(x, y)| {
                sync_attribute(x, y, dom_ref, tag);
            });
    } else {
        for old_key in xs.borrow().keys() {
            dom_ref.remove_attribute(old_key.as_str());
        }
        for (new_key, new_value) in ys.iter() {
            match new_value {
                Either::Left(true) => dom_ref.set_attribute(new_key, ""),
                Either::Right(value) => dom_ref.set_attribute(new_key, value),
                _ => ()
            }
            sync_stateful_attribute_dom_values(
                tag,
                dom_ref,
                new_key,
                &Either::Left(false),
                new_value
            );
        }
        xs.replace(ys.clone());
    }
}

pub fn sync_styling(
    x: &RefCell<StyleNode>,
    y: &StyleNode,
    node_id: &String
)
{
    let unchanged = {
        let x: &StyleNode = &x.borrow();
        x == y
    };
    if !unchanged {
        GLOBAL_CSS.with(|css| {
            css.add_node(node_id.clone(), y.clone());
        });
        x.replace(y.clone());
    }
}

pub fn sync_children<Msg>(xs: &RefCell<Vec<LiveHtml<Msg>>>, ys: &Vec<HtmlBuild<Msg>>, dom_ref: &DomRef)
where
    Msg: Clone + Debug + 'static
{
    let equal_length = {
        xs.borrow().len() == ys.len()
    };
    if equal_length {
        xs  .borrow_mut()
            .iter_mut()
            .zip(ys.iter())
            .for_each(|(x, y)| {
                if let Some(replacement) = x.sync(y) {
                    dom_ref.replace_child(replacement.dom_ref(), x.dom_ref());
                    *x = replacement;
                }
            });
    } else {
        for old_child in xs.borrow().iter() {
            dom_ref.remove_child(old_child.dom_ref());
        }
        let new_children: Vec<LiveHtml<Msg>> = {
            ys  .iter()
                .map(|y| {
                    let new_child = LiveHtml::from_builder(y.clone());
                    dom_ref.append_child(new_child.dom_ref());
                    new_child
                })
                .collect::<Vec<LiveHtml<Msg>>>()
        };
        xs.replace(new_children);
    }
}


pub fn form_reset<Msg>(node: &LiveNode<Msg>) {
    if node.tag == "form" {
        let node_ref: JsValue = From::from(node.dom_ref.dom_ref_as_element.clone());
        let node_ref: web_sys::HtmlFormElement = From::from(node_ref);
        node_ref.reset();
    }
}

pub fn set_input_value(dom_ref: &DomRef, value: &str) {
    let node_ref: JsValue = From::from(dom_ref.dom_ref_as_element.clone());
    let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
    node_ref.set_value(value);
}

