

use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::model_sys::incremental::{IVecSub, IVecSync};
use crate::view_sys::dsl::{self as dsl, Dsl, View};
use crate::view_sys::shared::*;
use crate::view_sys::extras::{DomThunk, EvalDomThunk};
use crate::view_sys::dom::*;
use crate::view_sys::runtime::common::*;



impl<Msg> Dom<Msg> {
    pub fn unsafe_tick_root(&mut self) {
        let tick_element = |element: &mut Element<Msg>| {
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
            tick_node_segment(segment, &new_env);
        };
        match self {
            Dom::Element(element) => tick_element(element),
            _ => panic!()
        }
    }
    pub fn tick(&mut self, env: &ElementEnv) {
        match self {
            Dom::Text(text) => {
                text.value.if_changed(|value: &String| {
                    text.dom_ref.set_text_content(value);
                });
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
                tick_node_segment(segment, &new_env);
            }
            Dom::Mixin(mixin) => {
                // SETUP
                let segment = NodeSegment {
                    attributes: &mut mixin.attributes,
                    events: &mut mixin.events,
                    children: &mut mixin.children,
                };
                // GO
                tick_node_segment(segment, env);
            }
            Dom::Control(Control::Toggle(toggle)) => {
                toggle.pred.if_changed(|new_pred| {
                    if new_pred.clone() {
                        let new_dom = toggle.template.build(env);
                        let old = toggle.dom.replace(Some(new_dom));
                        assert!(old.is_none());
                    }
                    else {
                        let old_dom = toggle.dom.replace(None);
                        assert!(old_dom.is_some());
                        if let Some(old_dom) = old_dom {
                            old_dom.remove(env);
                        }
                    }
                });
            }
            Dom::Control(Control::Linked(sub)) => {
                sub.sync(IVecSync {
                    active: move |children: &mut Vec<DomThunk<Msg>>| {
                        for child in children.iter_mut().rev() {
                            child.eval(EvalDomThunk {
                                new: |view: View<Msg>| -> Dom<Msg> {
                                    view.build(env)
                                },
                                update: |dom: &mut Dom<Msg>| {
                                    dom.tick(&env);
                                },
                            });
                            child.inspect(&mut |dom| {
                                env.rightward.replace(dom.get_before_dom_ref());
                            });
                        }
                    },
                    remove: move |dom: Dom<Msg>| {
                        dom.remove(env);
                    },
                });
            }
            Dom::Component(component) => {unimplemented!()}
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

fn tick_node_segment<'a, Msg>(segment: NodeSegment<'a, Msg>, env: &ElementEnv<'a>) {
    // ATTRIBUTES
    for (key, value) in segment.attributes.iter() {
        update_attribute(key, value, env);
    }
    // CHILDREN
    for child in segment.children.iter_mut().rev() {
        child.tick(&env);
        env.rightward.replace(child.get_before_dom_ref());
    }
}




