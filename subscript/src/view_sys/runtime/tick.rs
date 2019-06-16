

use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::reactive_sys::*;
use crate::view_sys::dsl::{self as dsl, Dsl, View};
use crate::view_sys::shared::*;
use crate::view_sys::extras::{DomThunk, EvalDomThunk};
use crate::view_sys::dom::*;
use crate::view_sys::runtime::common::*;
use crate::program_sys::spec::*;
use crate::program_sys::instances::TickEnv;



impl<Msg: 'static> Dom<Msg> {
    pub fn unsafe_tick_root(&mut self, tick_env: &mut TickEnv<Msg>) {
        let mut tick_element = |element: &mut Element<Msg>| {
            // SETUP
            let new_env = ElementEnv {
                tag: &element.tag,
                dom_ref: &element.dom_ref,
                rightward: &RefCell::new(None),
            };
            let segment = NodeSegment {
                attributes: &mut element.attributes,
                events: &mut element.events,
                children: &mut element.children,
            };
            // GO
            tick_node_segment(segment, &new_env, tick_env)
        };
        match self {
            Dom::Element(element) => tick_element(element),
            _ => panic!()
        }
    }
    pub fn tick(&mut self, env: &ElementEnv, tick_env: &mut TickEnv<Msg>) {
        match self {
            Dom::Text(text) => {
                text.value.if_changed(|value: &String| {
                    text.dom_ref.set_text_content(value);
                });
                // DONE
                env.rightward.replace(Some(Box::new(text.dom_ref.clone())));
            }
            Dom::Element(element) => {
                // SETUP
                let new_env = ElementEnv {
                    tag: &element.tag,
                    dom_ref: &element.dom_ref,
                    rightward: &RefCell::new(None),
                };
                let segment = NodeSegment {
                    attributes: &mut element.attributes,
                    events: &mut element.events,
                    children: &mut element.children,
                };
                // GO
                tick_node_segment(segment, &new_env, tick_env);
                // DONE
                env.rightward.replace(Some(Box::new(element.dom_ref.clone())));
            }
            Dom::Mixin(mixin) => {
                // SETUP
                let segment = NodeSegment {
                    attributes: &mut mixin.attributes,
                    events: &mut mixin.events,
                    children: &mut mixin.children,
                };
                // GO
                tick_node_segment(segment, env, tick_env)
            }
            Dom::Control(Control::Toggle(toggle)) => {
                if toggle.pred.get().as_ref().clone() {
                    let current = toggle.dom.replace(None);
                    if let Some(mut dom) = current {
                        dom.tick(env, tick_env);
                        toggle.dom.replace(Some(dom));
                    } else {
                        let new_dom = toggle.template.build(env);
                        toggle.dom.replace(Some(new_dom));
                    }
                } else {
                    let current = toggle.dom.replace(None);
                    if let Some(dom) = current {
                        dom.remove(env);
                    }
                }
            }
            Dom::Control(Control::Linked(observer)) => {
                use crate::reactive_sys::vec::view_observer::{TickArgs, ViewItem};
                observer.tick(TickArgs {
                    removed: &mut |dom: Dom<Msg>| {
                        dom.remove(env);
                    },
                    update: &mut |segment: &mut Vec<ViewItem<Msg>>| {
                        for mut child in segment.iter_mut().rev() {
                            match child {
                                ViewItem::View(view) => {
                                    let dom = view.build(env);
                                    *child = ViewItem::Dom(dom);
                                }
                                ViewItem::Dom(dom) => {
                                    dom.tick(&env, tick_env);
                                }
                            }
                        }
                    },
                });
            }
            Dom::Component(component) => {
                component.tick(tick_env.system_messages);
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// UTILS
///////////////////////////////////////////////////////////////////////////////

struct NodeSegment<'a, Msg> {
    attributes: &'a mut HashMap<String, Either<Value<String>, Value<bool>>>,
    events: &'a mut Vec<LiveEventHandler<Msg>>,
    children: &'a mut Vec<Dom<Msg>>,
}

fn tick_node_segment<'a, Msg: 'static>(segment: NodeSegment<'a, Msg>, env: &ElementEnv<'a>, tick_env: &mut TickEnv<Msg>) {
    // EVENTS
    for handler in segment.events {
        handler.tick(tick_env);
    }
    // ATTRIBUTES
    for (key, value) in segment.attributes.iter() {
        update_attribute(key, value, env);
    }
    // CHILDREN
    for child in segment.children.iter_mut().rev() {
        child.tick(&env, tick_env);
    }
}




