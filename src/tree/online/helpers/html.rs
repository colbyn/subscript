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
use crate::process::data::*;



pub fn unchanged<Msg: Clone>(x: &LiveHtml<Msg>, y: &HtmlBuild<Msg>) -> bool {
    match (x, y) {
        (LiveHtml::Node(x), HtmlBuild::Node(y)) => {
            let eq_tag = x.tag == y.tag;
            let eq_attributes: bool = {
                let ats: &HashMap<String, Either<bool, String>> =
                    &x.attributes.borrow();
                ats == &y.attributes
            };
            let eq_styling: bool = {
                let ss: &StyleNode = &x.styling.borrow();
                ss == &y.styling
            };
            let eq_events: bool = {
                let es: &HashMap<EventType, Callback<Msg>> =
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
            x.process.process_id() == y.process.process_id()
        }
        _ => {false}
    }
}

pub fn sync_events<Msg: Clone>(
    xs: &RefCell<HashMap<EventType, Callback<Msg>>>,
    ys: &HashMap<EventType, Callback<Msg>>,
    dom_ref: &DomRef
)
{
    let zipable = {
        let xs: &HashMap<String, Callback<Msg>> = &xs.borrow();
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

pub fn sync_attributes(
    xs: &RefCell<HashMap<String, Either<bool, String>>>,
    ys: &HashMap<String, Either<bool, String>>,
    dom_ref: &DomRef
)
{
    let zipable = {
        let xs: &HashMap<String, Either<bool, String>> = &xs.borrow();
        xs.keys().collect::<Vec<&String>>() == ys.keys().collect::<Vec<&String>>()
    };
    if zipable {
        xs  .borrow_mut()
            .iter_mut()
            .zip(ys.clone().iter_mut())
            .for_each(|(x, y)| {
                let unchanged = x.0 == y.0;
                if !unchanged {
                    *x.1 = y.1.clone();
                    match &x.1 {
                        Either::Left(true) => dom_ref.set_attribute(&x.0, ""),
                        Either::Left(false) => dom_ref.remove_attribute(&x.0),
                        Either::Right(value) => dom_ref.set_attribute(&x.0, value),
                    }
                }
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
                    x.clear();
                    replacement.init();
                    dom_ref.replace_child(replacement.dom_ref(), x.dom_ref());
                    *x = replacement;
                }
            });
    } else {
        for old_child in xs.borrow().iter() {
            old_child.clear();
            dom_ref.remove_child(old_child.dom_ref());
        }
        let new_children: Vec<LiveHtml<Msg>> = {
            ys  .iter()
                .map(|y| {
                    let new_child = LiveHtml::from_builder(y.clone());
                    new_child.init();
                    dom_ref.append_child(new_child.dom_ref());
                    new_child
                })
                .collect::<Vec<LiveHtml<Msg>>>()
        };
        xs.replace(new_children);
    }
}


