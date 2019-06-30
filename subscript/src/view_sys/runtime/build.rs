use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use wasm_bindgen::JsValue;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::backend::browser::{NodeApi, ElementApi, CallbackSettings, QueueCallback, VoidCallback};
use crate::reactive_sys::*;
use crate::view_sys::dsl::{self as dsl, Dsl, View};
use crate::view_sys::shared::*;
use crate::view_sys::dom::*;
use crate::view_sys::runtime::common::*;
use crate::view_sys::runtime::css as css_runtime;
use crate::program_sys::instances::*;
use crate::view_sys::runtime::css;


impl<Msg: 'static> View<Msg> {
    pub(crate) fn build_component(self, is_root: bool) -> Dom<Msg> {
        let build_root = |view: View<Msg>| -> Dom<Msg> {
            let tag = String::from("app");
            // let tag = if is_root {
            //     String::from("app")
            // } else {
            //     String::from("component")
            // };
            let dom_ref = browser::window().document.create_element(tag.as_str());
            css_runtime::register_defaults(&dom_ref);
            let new_env = ElementEnv {
                tag: tag.as_str(),
                dom_ref: &dom_ref,
                rightward: &RefCell::new(None),
            };
            let DomSegment{styling,attributes,events, children} = build_dom_segment(&new_env, ViewSegment {
                placement: SelfPlacement::Direct,
                styling: &Default::default(),
                attributes: &Default::default(),
                events: &Default::default(),
                children: &vec![view],
            });
            Dom::Element(Element {styling,dom_ref,auto_listeners: Vec::new(),tag,attributes,events,children})
        };
        build_root(self)
    }
    pub(crate) fn build(&self, env: &ElementEnv) -> Dom<Msg> {
        let window = browser::window();
        match &self.0 {
            Dsl::Text(text) => {
                let dom_ref = window.document
                    .create_text_node(text.0.get().as_str());
                insert_child(&dom_ref, env);
                env.rightward.replace(Some(Box::new(dom_ref.clone())));
                Dom::Text(Text{value: text.0.clone(), dom_ref})
            }
            Dsl::Element(x) => {
                let tag = x.tag.clone();
                let dom_ref = browser::window().document.create_element(tag.as_str());
                css_runtime::register_defaults(&dom_ref);
                let new_env = ElementEnv {
                    tag: tag.as_str(),
                    dom_ref: &dom_ref,
                    rightward: &RefCell::new(None),
                };
                let auto_listeners = get_and_add_auto_listeners::<Msg>(&new_env);
                let DomSegment{styling,attributes,events, children} = build_dom_segment(&new_env, ViewSegment {
                    placement: SelfPlacement::Direct,
                    styling: &x.styling,
                    attributes: &x.attributes,
                    events: &x.events,
                    children: &x.children,
                });
                insert_child(&dom_ref, env);
                env.rightward.replace(Some(Box::new(dom_ref.clone())));
                Dom::Element(Element {styling,dom_ref,auto_listeners,tag,attributes,events,children})
            }
            Dsl::Component(x) => {
                let process = LiveComponent(x.build());
                {
                    let dom_ref = process.dom_ref();
                    insert_child(&dom_ref, env);
                    env.rightward.replace(Some(Box::new(dom_ref.clone())));
                }
                Dom::Component(process)
            }
            Dsl::Mixin(x) => {
                let DomSegment{styling,attributes,events, children} = build_dom_segment(env, ViewSegment {
                    placement: SelfPlacement::Child,
                    styling: &x.styling,
                    attributes: &x.attributes,
                    events: &x.events,
                    children: &x.children,
                });
                Dom::Mixin(Mixin {styling,attributes,events,children})
            }
            Dsl::Control(dsl::Control::Toggle{pred, value}) => {
                let mut dom: Option<Dom<Msg>> = None;
                if pred.get().as_ref().clone() {
                    dom = Some(value.build(env));
                }
                Dom::Control(Control::Toggle(Box::new(Toggle{
                    pred: pred.clone(),
                    template: value.clone(),
                    dom: RefCell::new(dom),
                })))
            }
            Dsl::Control(dsl::Control::Linked(sub)) => {
                sub.build(&|children| {
                    let results = build_dom_segment(env, ViewSegment{
                        placement: SelfPlacement::Child,
                        styling: &Styling::default(),
                        attributes: &HashMap::new(),
                        events: &Vec::new(),
                        children: &children,
                    });
                    results.children
                });
                Dom::Control(Control::Linked(sub.clone()))
            }
            Dsl::Control(dsl::Control::Dynamic{producer}) => {
                let view = if let Some(view) = (producer.0)() {
                    let dom = view.build(env);
                    Some(dom)
                } else {
                    None
                };
                Dom::Control(Control::Dynamic(Box::new(Dynamic {
                    producer: producer.clone(),
                    view,
                })))
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// UTILS
///////////////////////////////////////////////////////////////////////////////

struct ViewSegment<'a, Msg> {
    placement: SelfPlacement,
    styling: &'a Styling,
    attributes: &'a HashMap<String, Either<Value<String>, Value<bool>>>,
    events: &'a Vec<EventHandler<Msg>>,
    children: &'a Vec<View<Msg>>,
}

struct DomSegment<Msg> {
    styling: Styling,
    attributes: HashMap<String, Either<Value<String>, Value<bool>>>,
    events: Vec<LiveEventHandler<Msg>>,
    children: Vec<Dom<Msg>>,
}

fn build_dom_segment<'a, Msg: 'static>(env: &ElementEnv<'a>, view_segment: ViewSegment<Msg>) -> DomSegment<Msg> {
    // SETUP
    let ViewSegment{placement,styling,attributes,events,children} = view_segment;
    let mut dom_segment = DomSegment {
        styling: styling.clone(),
        attributes: HashMap::new(),
        events: Vec::new(),
        children: Vec::new(),
    };
    // STYLING
    if !styling.is_empty() {
        let styling_env = css::upsert(&styling, placement.clone());
        env.dom_ref.class_list.add(&css::css_id_format(styling_env.css_id, &placement));
    }
    // ATTRIBUTES
    for (key, value) in attributes.iter() {
        set_attribute(&key, &value, env);
        dom_segment.attributes.insert(key.to_string(), value.clone());
    }
    // EVENTS
    for event in events.iter() {
        let backend_callback = QueueCallback::new(
            env.dom_ref,
            event.event_type().as_str(),
            CallbackSettings {
                prevent_default: {
                    event.event_type() == "submit"
                },
                ..CallbackSettings::default()
            },
        );
        dom_segment.events.push(LiveEventHandler{
            backend_callback,
            frontend_callback: event.clone(),
        });
    }
    // CHILDREN
    for new_child in children.into_iter().rev() {
        let child = new_child.build(env);
        dom_segment.children.insert(0, child);
    }
    // DONE
    dom_segment
}



pub(crate) fn get_and_add_auto_listeners<'a, Msg>(env: &ElementEnv<'a>) -> Vec<browser::VoidCallback> {
    let mut auto_listeners = Vec::new();
    let mut add_prevent_default = |event_type: &str| {
        let settings = CallbackSettings{prevent_default: true, ..CallbackSettings::default()};
        let callback = browser::VoidCallback::new(env.dom_ref, "submit", settings);
        auto_listeners.push(callback);
    };
    match env.tag {
        "form" => add_prevent_default("submit"),
        "a" => add_prevent_default("click"),
        _ => {}
    }
    auto_listeners
}




