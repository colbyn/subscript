use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};
use wasm_bindgen::JsValue;

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::reactive_sys::*;
use crate::view_sys::dsl::{self as dsl, Dsl, View};
use crate::view_sys::shared::*;
use crate::view_sys::dom::*;
use crate::program_sys::instances::SubProcessImpl;


pub(crate) struct ElementEnv<'a> {
    pub tag: &'a str,
    pub dom_ref: &'a browser::Element,
    pub rightward: &'a RefCell<Option<Box<browser::NodeApi>>>,
}

impl<'a> ElementEnv<'a> {
    pub(crate) fn get_rightward(&self) -> Option<Box<browser::NodeApi>> {
        let inner: &Option<Box<browser::NodeApi>> = &self.rightward.borrow();
        match inner {
            Some(x) => Some(x.box_clone()),
            None => None
        }
    }
}

pub(crate) fn insert_child<'a>(new: &browser::NodeApi, env: &ElementEnv<'a>) {
    if let Some(rightward) = env.get_rightward() {
        env.dom_ref.insert_before(new, rightward.as_ref());
    } else {
        env.dom_ref.append_child(new);
    }
}

impl<Msg> Dom<Msg> {
    pub(crate) fn unsafe_remove_root(self) {
        match self {
            Dom::Element(element) => {
                let env = ElementEnv {
                    tag: &element.tag,
                    dom_ref: &element.dom_ref,
                    rightward: &RefCell::new(None),
                };
                let styling_env = crate::view_sys::runtime::css::removed(&element.styling);
                element.dom_ref.class_list.remove(&styling_env.css_id());
                for event in element.events.iter() {
                    env.dom_ref.remove_event_listener(&event.event_type(), &event.backend_callback);
                }
                for child in element.children {
                    child.remove(&env);
                }
            }
            _ => panic!()
        }
    }
    pub(crate) fn remove<'a>(self, env: &ElementEnv<'a>) {
        match self {
            Dom::Component(value) => {
                env.dom_ref.remove_child(&value.dom_ref());
            }
            Dom::Text(value) => {
                env.dom_ref.remove_child(&value.dom_ref);
            }
            Dom::Element(value) => {
                let new_env = ElementEnv {
                    tag: &value.tag,
                    dom_ref: &value.dom_ref,
                    rightward: &RefCell::new(None),
                };
                let styling_env = crate::view_sys::runtime::css::removed(&value.styling);
                value.dom_ref.class_list.remove(&styling_env.css_id());
                for event in value.events.iter() {
                    env.dom_ref.remove_event_listener(&event.event_type(), &event.backend_callback);
                }
                for child in value.children {
                    child.remove(&new_env);
                }
                env.dom_ref.remove_child(&value.dom_ref);
            }
            Dom::Mixin(value) => {
                let styling_env = crate::view_sys::runtime::css::removed(&value.styling);
                env.dom_ref.class_list.remove(&styling_env.css_id());
                for (key, value) in value.attributes.iter() {
                    remove_attribute(key, value, env);
                }
                for event in value.events.iter() {
                    env.dom_ref.remove_event_listener(&event.event_type(), &event.backend_callback);
                }
                for child in value.children {
                    child.remove(env);
                }
            }
            Dom::Control(Control::Toggle(toggle)) => {
                let inner: Option<Dom<Msg>> = toggle.dom.into_inner();
                if let Some(dom) = inner {
                    dom.remove(env);
                }
            }
            Dom::Control(Control::Linked(observer)) => {
                use crate::reactive_sys::vec::view_observer::ViewItem;
                if let Right(dom) = observer.terminate() {
                    for child in dom.removed {
                        child.remove(env);
                    }
                    for node in dom.active {
                        if let ViewItem::Dom(dom) = node {
                            dom.remove(env);
                        }
                    }
                }
            }
        }
    }
}


pub(crate) fn set_attribute<'a>(key: &String, value: &Either<Value<String>, Value<bool>>, element: &ElementEnv<'a>) {
    let class_attribute = || match value {
        Left(value) => {
            for cls in value.get().split(' ') {
                element.dom_ref.class_list.add(cls);
            }
        }
        _ => {}
    };
    let attribute = || match value {
        Left(value) => {
            element.dom_ref.set_attribute(key.as_str(), value.get().as_str());
        }
        Right(value) => {
            if value.get().as_ref().clone() {
                element.dom_ref.set_attribute(key.as_str(), "");
            } else {
                element.dom_ref.remove_attribute(key.as_str());
            }
        }
    };
    let value_property = || {
        match value {
            Left(value) => {
                let node_ref: JsValue = element.dom_ref.dom_ref();
                let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
                node_ref.set_value(&value.get());
            }
            _ => ()
        }
    };
    let checked_property = || {
        match value {
            Right(value) => {
                let node_ref: JsValue = element.dom_ref.dom_ref();
                let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
                node_ref.set_checked(value.get().as_ref().clone());
            }
            _ => ()
        }
    };
    match (element.tag, key.as_str()) {
        ("input", "value") => value_property(),
        ("input", "checked") => checked_property(),
        (_, "class") => class_attribute(),
        _ => attribute(),
    }
}

pub(crate) fn update_attribute<'a>(key: &String, value: &Either<Value<String>, Value<bool>>, element: &ElementEnv<'a>) {
    let class_attribute = || match value {
        Left(string) => {
            string.if_changed_with_old(|data| {
                let IfChangedWithOld{old, new} = data;
                for cls in old.split(' ') {
                    element.dom_ref.class_list.remove(cls);
                }
                for cls in new.split(' ') {
                    element.dom_ref.class_list.add(cls);
                }
            });
        }
        _ => {}
    };
    let attribute = || match value {
        Left(string) => {
            string.if_changed(|new_value| {
                element.dom_ref.set_attribute(key.as_str(), new_value.as_str());
            });
        }
        Right(boolean) => {
            boolean.if_changed(|new_value| {
                if new_value.clone() {
                    element.dom_ref.set_attribute(key.as_str(), "");
                } else {
                    element.dom_ref.remove_attribute(key.as_str());
                }
            }); 
        }
    };
    let value_property = || {
        match value {
            Left(value) => {
                value.if_changed(|new_value| {
                    let node_ref: JsValue = element.dom_ref.dom_ref();
                    let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
                    node_ref.set_value(new_value);
                });
            }
            _ => ()
        }
    };
    let checked_property = || {
        match value {
            Right(boolean) => {
                boolean.if_changed(|new_value| {
                    let node_ref: JsValue = element.dom_ref.dom_ref();
                    let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
                    node_ref.set_checked(new_value.clone());
                });
            }
            _ => ()
        }
    };
    match (element.tag, key.as_str()) {
        ("input", "value") => value_property(),
        ("input", "checked") => checked_property(),
        (_, "class") => class_attribute(),
        _ => attribute(),
    }
}

pub(crate) fn remove_attribute<'a>(key: &String, value: &Either<Value<String>, Value<bool>>, element: &ElementEnv<'a>) {
    let class_attribute = || match value {
        Left(string) => {
            match string {
                Value::Dynamic(dynamic) => {
                    let old = dynamic.current.borrow();
                    for cls in old.split(' ') {
                        element.dom_ref.class_list.remove(cls);
                    }
                }
                Value::Static(value) => {
                    for cls in value.split(' ') {
                        element.dom_ref.class_list.remove(cls);
                    }
                }
            }
        }
        _ => {}
    };
    let attribute = || {
        element.dom_ref.remove_attribute(key.as_str());
    };
    match (element.tag, key.as_str()) {
        ("input", "value") => {}
        ("input", "checked") => {}
        (_, "class") => class_attribute(),
        _ => attribute(),
    }
}

impl<Msg: 'static> Dom<Msg> {
    pub(crate) fn get_first_dom_ref(&self) -> Option<Box<browser::NodeApi>> {
        fn check_children<Msg: 'static>(children: &Vec<Dom<Msg>>) -> Option<Box<browser::NodeApi>> {
            let mut result: Option<Box<browser::NodeApi>> = None;
            for child in children.iter() {
                if result.is_none() {
                    result = child.get_first_dom_ref();
                }
            }
            result
        }
        match self {
            Dom::Control(Control::Linked(observer)) => {
                let mut result: Option<Box<browser::NodeApi>> = None;
                observer.for_each_dom_node(&mut |node: &Dom<Msg>| {
                    if result.is_none() {
                        result = node.get_first_dom_ref();
                    }
                });
                result
            }
            Dom::Control(Control::Toggle(toggle)) => {
                let mut result = None;
                if toggle.pred.get().as_ref().clone() {
                    let inner: &Option<Dom<Msg>> = &toggle.dom.borrow();
                    assert!(inner.is_some());
                    if let Some(dom) = inner {
                        result = dom.get_first_dom_ref();
                    }
                }
                result
            }
            Dom::Mixin(x) => check_children(&x.children),
            Dom::Component(x) => Some(Box::new(x.dom_ref())),
            Dom::Text(x) => Some(x.dom_ref.box_clone()),
            Dom::Element(x) => Some(x.dom_ref.box_clone()),
        }
    }
}

