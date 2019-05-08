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
use crate::tree::online::helpers::{self, html};
use crate::process::data::*;


///////////////////////////////////////////////////////////////////////////////
// INTERFACE - CSS
///////////////////////////////////////////////////////////////////////////////


impl GlobalCss {
    pub fn tick(&self) {
        let unchanged = {
            let x: &BTreeMap<NodeId, StyleNode> = &self.offline_nodes.borrow();
            let y: &BTreeMap<NodeId, StyleNode> = &self.online_nodes.borrow();
            x == y
        };
        if !unchanged {
            let rendered: String = {
                let ref offline_nodes = self.offline_nodes.borrow();
                helpers::css::render_style_nodes_tree(offline_nodes)
            };
            self.online_nodes.replace(self.offline_nodes.borrow().clone());
            self.dom_ref.set_text_content(rendered.as_str());
        }
    }
    pub fn add_node(&self, node_id: String, node: StyleNode) {
        self.offline_nodes
            .borrow_mut()
            .insert(node_id, node);
    }
    pub fn remove_node(&self, node_id: &str) {
        self.offline_nodes
            .borrow_mut()
            .remove(node_id);
    }
}




///////////////////////////////////////////////////////////////////////////////
// INTERFACE - HTML
///////////////////////////////////////////////////////////////////////////////

impl<Msg: Clone + Debug + 'static> LiveHtml<Msg> {
    pub fn from_builder(build: HtmlBuild<Msg>) -> Self {
        match build {
            HtmlBuild::Component(comp) => {
                let component = LiveHtml::Component(LiveComponent {
                    process: comp.process.clone(),
                });
                component.online();
                component
            },
            HtmlBuild::Text(text) => {
                LiveHtml::Text(LiveText {
                    dom_ref: DomRef::new_text(text.value.as_str()),
                    value: RefCell::new(text.value),
                })
            }
            HtmlBuild::Node(node) => {
                let dom_ref = DomRef::new(node.tag.as_str());
                let node_id = format!("id-{}", rand::random::<u16>());
                let tag = node.tag;
                let attributes = {
                    dom_ref.set_attribute(node_id.as_str(), "");
                    for (key, value) in node.attributes.iter() {
                        match value {
                            Either::Left(false) => (),
                            Either::Left(true) => {
                                dom_ref.set_attribute(key, "");
                            },
                            Either::Right(value) => {
                                dom_ref.set_attribute(key, value);
                            },
                        }
                    }
                    RefCell::new(node.attributes)
                };
                let events = {
                    for (key, value) in node.events.iter() {
                        dom_ref.add_event_listener(key, &value.js_function);
                    }
                    RefCell::new(node.events)
                };
                let styling = {
                    let node_id = node_id.clone();
                    let styling = node.styling.clone();
                    GLOBAL_CSS.with(|css| {
                        css.add_node(node_id, styling);
                    });
                    RefCell::new(node.styling)
                };
                let children = RefCell::new({
                    let mut xs: Vec<LiveHtml<Msg>> = Vec::new();
                    for child in node.children {
                        let live_child = LiveHtml::from_builder(child);
                        dom_ref.append_child(live_child.dom_ref());
                        xs.push(live_child);
                    }
                    xs
                });
                LiveHtml::Node(LiveNode {
                    dom_ref: dom_ref,
                    node_id: node_id,
                    tag: tag,
                    attributes: attributes,
                    events: events,
                    styling: styling,
                    children: children,
                })
            }
        }
    }
    pub fn dom_ref(&self) -> &DomRef {
        match self {
            LiveHtml::Node(node) => &node.dom_ref,
            LiveHtml::Text(text) => &text.dom_ref,
            LiveHtml::Component(comp) => &comp.process.dom_ref(),
        }
    }
}

impl<Msg: Clone + Debug + 'static> LiveHtml<Msg> {
    pub fn online(&self) {
        match self {
            LiveHtml::Node(node) => {
                for child in node.children.borrow().iter() {
                    child.online();
                }
            }
            LiveHtml::Component(comp) => {
                comp.process.online()
            },
            LiveHtml::Text(text) => (),
        }
    }
    pub fn offline(&self) {
        match self {
            LiveHtml::Node(node) => {
                for child in node.children.borrow().iter() {
                    child.offline();
                }
            }
            LiveHtml::Component(comp) => {
                comp.process.offline()
            },
            LiveHtml::Text(text) => (),
        }
    }
    pub fn tick(&self, messages: &mut Vec<Msg>, sub_enqueue: &Vec<Rc<Any>>) {
        match self {
            LiveHtml::Node(node) => {
                for child in node.children.borrow().iter() {
                    child.tick(messages, sub_enqueue);
                }
                for event in node.events.borrow().values() {
                    messages.append(&mut event.drain());
                }
            }
            LiveHtml::Component(comp) => {
                comp.process.tick(sub_enqueue);
            }
            LiveHtml::Text(text) => {}
        }
    }
    pub fn sync(&self, other: &HtmlBuild<Msg>) -> Option<LiveHtml<Msg>> {
        let unchanged = html::unchanged(&self, &other);
        if !unchanged {
            let mut replacement: Option<LiveHtml<Msg>> = None;
            match (self, other) {
                (LiveHtml::Node(x), HtmlBuild::Node(y)) => {
                    let tag_unchanged = x.tag == y.tag;
                    if tag_unchanged {
                        html::sync_attributes(&x.attributes, &y.attributes, &x.dom_ref, &x.tag);
                        html::sync_events(&x.events, &y.events, &x.dom_ref);
                        html::sync_styling(&x.styling, &y.styling, &x.node_id);
                        html::sync_children(&x.children, &y.children, &x.dom_ref);
                    } else {
                        replacement = Some(LiveHtml::from_builder(other.clone()));
                    }
                }
                (LiveHtml::Component(x), HtmlBuild::Component(y)) => {
                    replacement = Some(LiveHtml::from_builder(other.clone()));
                },
                (LiveHtml::Text(x), HtmlBuild::Text(y)) => {
                    x.dom_ref.set_text_content(&y.value);
                    x.value.replace(y.value.clone());
                }
                _ => {
                    replacement = Some(LiveHtml::from_builder(other.clone()));
                }
            }
            replacement
        } else {
            None
        }
    }
}

