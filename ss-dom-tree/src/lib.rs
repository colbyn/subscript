#![allow(dead_code, unused, unused_variables)]
pub mod css;

use std::fmt::Debug;
use std::cell::*;
use std::convert::From;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;
use std::any::*;
use std::collections::*;
use either::Either::{self, Left, Right};
use wasm_bindgen::JsValue;

use ss_web_utils::prelude::*;
use ss_web_utils::dom;
use ss_web_utils::js::{self, console, QueueCallback, VoidCallback};
use ss_trees::tree::*;
use ss_trees::ext::map::{SMap, MapApi};
use ss_view_tree::*;
use ss_view_tree::events::*;
use ss_view_tree::attributes::*;
use ss_view_tree::styling::{CssId, Stylesheet, Style, MediaQuerySelector, StateSelector};


///////////////////////////////////////////////////////////////////////////////
// LIVE-VIEW
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct LiveView<Msg> where Msg: PartialEq + Debug + Clone {
    sync_api: DomTreeLogic,
    mount: Meta,
    tree: STree<Meta, LiveNode<Msg>, LiveLeaf, ViewNode<Msg>, ViewLeaf>,
}

impl<Msg: 'static> LiveView<Msg> where Msg: PartialEq + Debug + Clone {
    pub fn start(initial_view: View<Msg>) -> Self {
        let window = dom::window();
        let sync_api = DomTreeLogic::default();
        let mount = Meta::Tag {
            dom_ref: {
                let mount = window.document.create_element("div");
                mount.set_attribute("view-wrapper", "");
                window.document.body.append_child(&mount);
                Rc::new(mount)
            },
        };
        let tree = STree::from(
            &sync_api,
            &mount,
            &initial_view.0
        );
        LiveView {sync_api,mount,tree}
    }
    pub fn sync(&mut self, view: View<Msg>) {
        self.tree.traverse_sync(&self.sync_api, &self.mount, &view.0, &SyncTraversal {
            leafs: &move |parent, n1, n2| {},
            nodes: &move |parent, n1, n2| {
                n1.events.borrow().traverse_values_pair(&n2.events, &move |e1, e2| {
                    e1.sync(e2);
                });
            },
            new_node: &move |parent, n| {},
            new_leaf: &move |parent, l| {},
        })
    }
    pub fn tick(&self, env: &RefCell<TickEnv<Msg>>, reg: &GlobalTickRegistry) {
        self.tree.traverse(&Traversal {
            node: &|node: &LiveNode<Msg>| -> () {
                node.events.borrow_mut().traverse_values_mut(|handler| {
                    let f: &EventHandler<Msg> = &handler.value.borrow();
                    env.borrow_mut().messages.append(
                        &mut handler.callback.drain()
                            .into_iter()
                            .map(|event| f.run_handler(event))
                            .collect::<Vec<Msg>>()
                    );
                });
            },
            leaf: &|leaf: &LiveLeaf| -> () {
                match leaf {
                    LiveLeaf::Text{value, ..} => {}
                    LiveLeaf::Component{value, ..} => {
                        reg.components.borrow_mut().push(value.tick(reg));
                    }
                }
            }
        });
    }
}

///////////////////////////////////////////////////////////////////////////////
// STYLING
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiveStylesheet {
    css_id: RefCell<u64>,
    value: RefCell<Stylesheet>,
}

impl LiveStylesheet {
    pub fn sync(&self, new: &Stylesheet, dom_ref: &dom::Tag) {
        let styling_unchanged = {
            &*self.value.borrow() == new
        };
        if !styling_unchanged {
            let css_id = calculate_hash(new);
            dom_ref.set_attribute("css", trim_css_id(&css_id).as_str());
            let new = new.clone();
            self.value.replace(new);
            css::upsert(&self);
        }
    }
    pub fn new(value: Stylesheet, dom_ref: &dom::Tag) -> Self {
        let css_id = RefCell::new(calculate_hash(&value));
        let x: u64 = calculate_hash(&value);
        let x: u32 = x as u32;
        let value = RefCell::new(value);
        let result = LiveStylesheet{value, css_id};
        if !result.value.borrow().is_empty() {
            let css_id: u64 = result.css_id.borrow().clone();
            dom_ref.set_attribute("css", trim_css_id(&css_id).as_str());
            css::upsert(&result);
        }
        result
    }
    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_empty()
    }
}

pub fn trim_css_id(x: &u64) -> String {
    let mut x = format!("{}", x);
    x
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////

pub type AttributesMap<Msg> = SMap<LiveNode<Msg>, String, AttributeValue, AttributeValue>;

#[derive(Debug, PartialEq, Clone)]
pub struct AttributesApi {}

pub fn upsert_attribute<Msg>(attached: &LiveNode<Msg>, key: &String, new: &AttributeValue)
where
    Msg: PartialEq + 'static + Debug + Clone
{
    match (attached.tag.as_str(), key.as_str()) {
        ("input", "value") => {
            if let Some(value) = new.get_string() {
                set_input_text_value(attached.dom_ref.as_ref(), value);
            }
        }
        ("input", "checked") => {
            if let Some(value) = new.get_string() {
                if value == "true" {
                    set_input_checked_value(attached.dom_ref.as_ref(), true);
                }
                if value == "false" {
                    set_input_checked_value(attached.dom_ref.as_ref(), false);
                }
            }
            if let Some(value) = new.get_bool() {
                set_input_checked_value(attached.dom_ref.as_ref(), value);
            }
        }
        _ => ()
    }
    // SET ATTRIBUTE
    match &new {
        AttributeValue::Value(str) => {
            attached.dom_ref.set_attribute(key, str)
        }
        AttributeValue::Toggle(state) if key == "checked" => {
            attached.dom_ref.set_attribute(key, format!("{}", state).as_str())
        }
        AttributeValue::Toggle(true) => {
            assert!(key != "checked");
            attached.dom_ref.set_attribute(key, "")
        }
        AttributeValue::Toggle(false) => {
            assert!(key != "checked");
        }
    }
}

impl<Msg> MapApi<LiveNode<Msg>, String, AttributeValue, AttributeValue> for AttributesApi
where
    Msg: PartialEq + 'static + Debug + Clone
{
    fn create(&self, attached: &LiveNode<Msg>, key: &String, new: AttributeValue) -> AttributeValue {
        upsert_attribute(attached, key, &new);
        new
    }
    fn modified(&self, attached: &LiveNode<Msg>, key: &String, old: &mut AttributeValue, new: AttributeValue) {
        upsert_attribute(attached, key, &new);
        *old = new;
    }
    fn remove(&self, attached: &LiveNode<Msg>, key: String, old: AttributeValue) {
        console::log("AttributesApi.remove");
    	attached.dom_ref.remove_attribute(&key);
    }
    fn unchanged(&self, old: &AttributeValue, new: &AttributeValue) -> bool {
    	old == new
    }
}


/// Helper for AttributesApi.
fn set_input_text_value(dom_ref: &dom::Tag, value: &str) {
    let node_ref: JsValue = From::from(dom_ref.dom_ref_as_element());
    let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
    node_ref.set_value(value);
}
fn set_input_checked_value(dom_ref: &dom::Tag, value: bool) {
    let node_ref: JsValue = From::from(dom_ref.dom_ref_as_element());
    let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
    node_ref.set_checked(value);
}


///////////////////////////////////////////////////////////////////////////////
// EVENTS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct LiveEventHandler<Msg> {
    pub value: RefCell<EventHandler<Msg>>,
    pub callback: js::DomEventHandler,
}

impl<Msg> PartialEq for LiveEventHandler<Msg> {
    fn eq(&self, other: &LiveEventHandler<Msg>) -> bool {
        self.value.borrow().event_name() == other.value.borrow().event_name()
    }
}

impl<'a, Msg: Clone> LiveEventHandler<Msg> {
    pub fn event_name(&self) -> EventType {
        self.value.borrow().event_name()
    }
    pub fn sync(&self, new: &EventHandler<Msg>) {
        self.value.replace(new.clone());
    }
}

pub type EventsMap<Msg> = SMap<LiveNode<Msg>, EventType, LiveEventHandler<Msg>, EventHandler<Msg>>;

#[derive(Debug, PartialEq, Clone)]
pub struct EventsApi {}

impl<'a, Msg> MapApi<LiveNode<Msg>, EventType, LiveEventHandler<Msg>, EventHandler<Msg>> for EventsApi
where
    Msg: PartialEq + 'static + Debug + Clone
{
    fn create(&self, attached: &LiveNode<Msg>, key: &EventType, new: EventHandler<Msg>) -> LiveEventHandler<Msg> {
        assert!({key == &new.event_name()});
        let x = dom::window();
        let value = RefCell::new(new);
        let prevent_default = match (attached.tag.as_str(), key) {
            ("form", EventType::OnSubmit) => true,
            ("input", EventType::OnSubmit) => true,
            _ => false,
        };
        let settings = js::DomCallbackSettings {
            stop_propagation: false,
            prevent_default,
        };
        let callback = js::DomEventHandler::new(settings);
        attached.dom_ref.add_event_listener(key.as_str(), &callback);
        LiveEventHandler {
            callback,
            value,
        }
    }
    fn modified(&self, attached: &LiveNode<Msg>, key: &EventType, old: &mut LiveEventHandler<Msg>, new: EventHandler<Msg>) {
        old.value.replace(new);
    }
    fn remove(&self, attached: &LiveNode<Msg>, key: EventType, old: LiveEventHandler<Msg>) {
        use ss_web_utils::dom::DomRef;
    	assert_eq!(key, old.event_name());
        attached.dom_ref.remove_event_listener(key.as_str(), &old.callback);
    }
    fn unchanged(&self, old: &LiveEventHandler<Msg>, new: &EventHandler<Msg>) -> bool {true}
}



///////////////////////////////////////////////////////////////////////////////
// LIVE DOM TREE
///////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Clone)]
pub enum LiveLeaf {
    Text {
        dom_ref: Rc<dom::Text>,
        value: String,
    },
    Component {
        dom_ref: Rc<dom::Tag>,
        value: Box<ViewComponent>,
    },
}


impl PartialEq for LiveLeaf {
    fn eq(&self, other: &LiveLeaf) -> bool {
        match (self, other) {
            (LiveLeaf::Text{value: v1, ..}, LiveLeaf::Text{value: v2, ..}) => {v1 == v2}
            (LiveLeaf::Component{value: v1, ..}, LiveLeaf::Component{value: v2, ..}) => {v1 == v2}
            _ => false
        }
    }
}

impl LiveLeaf {
    pub fn get_meta(&self) -> Meta {
        match self {
            LiveLeaf::Text{dom_ref, ..} => {
                let dom_ref = dom_ref.clone();
                Meta::Text{dom_ref}
            }
            LiveLeaf::Component{dom_ref, ..} => {
                let dom_ref = dom_ref.clone();
                Meta::Tag{dom_ref}
            }
        }
    }
    pub fn is_text(&self) -> bool {
        match self {
            LiveLeaf::Text{..} => true,
            _ => false,
        }
    }
    pub fn is_component(&self) -> bool {
        match self {
            LiveLeaf::Component{..} => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiveNode<Msg> where Msg: PartialEq + Clone + Debug {
    pub tag: String,
    pub auto_listeners: Rc<HashMap<String, VoidCallback>>,
    pub dom_ref: Rc<dom::Tag>,
    pub attributes: Rc<RefCell<AttributesMap<Msg>>>,
    pub events: Rc<RefCell<EventsMap<Msg>>>,
    pub styling: LiveStylesheet,
}

impl<Msg> Drop for LiveNode<Msg> where Msg: PartialEq + Clone + Debug {
    fn drop(&mut self) {
        // css::remove(&self.styling.css_id);
        // let mut auto_listeners = Rc::make_mut(&mut self.auto_listeners);
        // for (event_name, callback) in auto_listeners.drain() {
        //     self.dom_ref.remove_event_listener(&event_name, &callback);
        // }
        // for (key, live_event_listener) in self.events.borrow_mut().dangerous_unsync_drain() {
        //     self.dom_ref.remove_event_listener(key.as_str(), &live_event_listener.callback);
        // }
    }
}

impl<Msg> LiveNode<Msg> where Msg: PartialEq + Clone + Debug {
    pub fn get_meta(&self) -> Meta {
        let dom_ref = self.dom_ref.clone();
        Meta::Tag{dom_ref}
    }
}

impl<Msg> PartialEq for LiveNode<Msg> where Msg: PartialEq + Clone + Debug {
    fn eq(&self, other: &LiveNode<Msg>) -> bool {
        self.tag == other.tag &&
        self.attributes == other.attributes &&
        self.events == other.events &&
        self.styling == other.styling
    }
}


///////////////////////////////////////////////////////////////////////////////
// SYNC API
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum DomRefType {
    Text,
    Tag,
}

#[derive(Debug, Clone)]
pub enum Meta {
    Text {
        dom_ref: Rc<dom::Text>,
    },
    Tag {
        dom_ref: Rc<dom::Tag>,
    }
}

impl PartialEq for Meta {
    fn eq(&self, other: &Meta) -> bool {
        match (self, other) {
            (Meta::Tag{..}, Meta::Tag{..}) => {true}
            (Meta::Text{..}, Meta::Text{..}) => {true}
            _ => false
        }
    }
}


impl Meta {
    pub fn get_dom_ref(&self) -> Rc<dom::DomRef> {
        match self {
            Meta::Text{dom_ref} => dom_ref.clone(),
            Meta::Tag{dom_ref} => dom_ref.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DomTreeLogic {
    pub window: dom::Window,
    pub attributes_api: AttributesApi,
    pub events_api: EventsApi,
}


impl PartialEq for DomTreeLogic {
    fn eq(&self, other: &DomTreeLogic) -> bool {true}
}


impl Default for DomTreeLogic {
    fn default() -> Self {
        DomTreeLogic {
            window: dom::window(),
            attributes_api: AttributesApi {},
            events_api: EventsApi {},
        }
    }
}

impl<Msg> TreeApi<Meta, LiveNode<Msg>, LiveLeaf, ViewNode<Msg>, ViewLeaf> for DomTreeLogic
where
    Msg: PartialEq + 'static + Debug + Clone
{
    fn node_unchanged(&self, new: &ViewNode<Msg>, old: &LiveNode<Msg>) -> bool {
        let equal_tags = new.tag == old.tag;
        let attributes_unchanged = || {
            old.attributes.borrow().unchanged(
                &self.attributes_api,
                &new.attributes,
            )
        };
        let events_unchanged = || {
            let mut ks: HashSet<EventType> = HashSet::new();
            for k in new.events.keys() {
                ks.insert(k.clone());
            }
            old.events.borrow().get_keys() == ks
        };
        let styling_unchanged = || {
            *old.styling.value.borrow() == new.styling
        };
        let result = equal_tags && styling_unchanged() && attributes_unchanged() && events_unchanged();
        result
    }
    fn node_recyclable(&self, new: &ViewNode<Msg>, old: &LiveNode<Msg>) -> bool {
        let equal_tags = new.tag == old.tag;
        let events_recyclable = || {
            let mut ks: HashSet<EventType> = HashSet::new();
            for k in new.events.keys() {
                ks.insert(k.clone());
            }
            old.events.borrow().get_keys() == ks
        };
        equal_tags && events_recyclable()
    }
    fn node_update(&self, update: Update<&mut LiveNode<Msg>, &ViewNode<Msg>>) {
        // SETUP
        let Update{new, old} = update;
        assert!(new.tag == old.tag);
        // UNCHANGED CHECKS
        let attributes_unchanged = old.attributes.borrow().unchanged(
            &self.attributes_api,
            &new.attributes,
        );
        let events_unchanged = {
            let mut ks: HashSet<EventType> = HashSet::new();
            for k in new.events.keys() {
                ks.insert(k.clone());
            }
            old.events.borrow().get_keys() == ks
        };
        let styling_unchanged = *old.styling.value.borrow() == new.styling;
        // SYNC CHANGES
        if !attributes_unchanged {
            let new_attributes: HashMap<String, AttributeValue> = new.attributes.clone();
            old.attributes.borrow_mut().sync(
                &self.attributes_api,
                &old,
                new_attributes,
            );
        }
        if !events_unchanged {
            let new_events: HashMap<events::EventType, EventHandler<Msg>> = new.events.clone();
            old.events.borrow_mut().sync(
                &self.events_api,
                &old,
                new_events,
            );
        }
        old.styling.sync(&new.styling, &old.dom_ref);
        // if !styling_unchanged {
        //     let new_styling: Stylesheet = new.styling.clone();
        //     old.styling.value.replace(new_styling);
        //     css::upsert(&old.styling);
        // }
    }
    fn node_crate(&self, new: &ViewNode<Msg>) -> LiveNode<Msg> {
        let new_attributes: HashMap<String, AttributeValue> = new.attributes.clone();
        let new_events: HashMap<events::EventType, EventHandler<Msg>> = new.events.clone();
        let new_styling: Stylesheet = new.styling.clone();
        let dom_ref = self.window.document.create_element(new.tag.as_str());
        let mut auto_listeners = HashMap::new();
        let mut add_prevent_default_callback = |event_name: &str| {
            let callback = move |event: JsValue| {
                let event: web_sys::Event = From::from(event);
                event.prevent_default();
            };
            let callback = VoidCallback::new(callback);
            dom_ref.add_event_listener(event_name, &callback);
            auto_listeners.insert(String::from(event_name), callback);
        };
        match new.tag.as_str() {
            "form" => add_prevent_default_callback("submit"),
            "input" => add_prevent_default_callback("submit"),
            "a" => add_prevent_default_callback("click"),
            _ => ()
        }
        let auto_listeners = Rc::new(auto_listeners);
        let result = LiveNode {
            auto_listeners,
            styling: LiveStylesheet::new(new_styling, &dom_ref),
            dom_ref: Rc::new(dom_ref),
            tag: new.tag.clone(),
            attributes: Rc::new(RefCell::new(SMap::default())),
            events: Rc::new(RefCell::new(SMap::default())),
        };
        result.attributes.borrow_mut().sync(
            &self.attributes_api,
            &result,
            new_attributes,
        );
        result.events.borrow_mut().sync(
            &self.events_api,
            &result,
            new_events,
        );
        result
    }
    fn leaf_unchanged(&self, new: &ViewLeaf, old: &LiveLeaf) -> bool {
        match (new, old) {
            (ViewLeaf::Text(x), LiveLeaf::Text{value, ..}) => x == value,
            (ViewLeaf::Component(new), LiveLeaf::Component{value: old, ..}) => {
                let new: Box<Any> = Box::new(new.clone());
                old.unchanged(&new)
            },
            _ => false
        }
    }
    fn leaf_recyclable(&self, new: &ViewLeaf, old: &LiveLeaf) -> bool {
        match (new, old) {
            (ViewLeaf::Text(_), LiveLeaf::Text{..}) => true,
            (ViewLeaf::Component(new), LiveLeaf::Component{value: old, ..}) => {
                let new: Box<Any> = Box::new(new.clone());
                old.recyclable(&new)
            },
            _ => false
        }
    }
    fn leaf_update(&self, update: Update<&mut LiveLeaf, &ViewLeaf>) {
        let Update{new, old} = update;
        match (&new, old) {
            (ViewLeaf::Text(new), LiveLeaf::Text{value: old, dom_ref}) => {
                dom_ref.set_text_content(new.as_str());
                *old = new.clone();
            }
            (ViewLeaf::Component(_), LiveLeaf::Component{value, dom_ref}) => {
                panic!("todo...")
            }
            _ => panic!()
        }
    }
    fn leaf_crate(&self, new: &ViewLeaf) -> LiveLeaf {
        use ss_web_utils::dom::DomRef;
        match new {
            ViewLeaf::Text(value) => {
                let dom_ref = self.window.document.create_text_node(value.as_str());
                dom_ref.set_text_content(value.as_str());
                LiveLeaf::Text {
                    dom_ref: Rc::new(dom_ref),
                    value: value.to_string(),
                }
            }
            ViewLeaf::Component(value) => {
                let dom_ref = self.window.document.create_element("div");
                dom_ref.set_attribute("component-wrapper", "");
                LiveLeaf::Component {
                    dom_ref: Rc::new(dom_ref),
                    value: value.clone(),
                }
            }
        }
    }

    fn get_meta(&self, value: Either<&LiveNode<Msg>, &LiveLeaf>) -> Meta {
        match value {
            Left(node) => {
                node.get_meta()
            }
            Right(leaf) => {
                leaf.get_meta()
            }
        }
    }
    fn insert(&self, op: InsertOp<Meta>) {
        fn init_fragment(new: Vec<Meta>) -> web_sys::DocumentFragment {
            let fragment = web_sys::DocumentFragment::new()
                .expect("new DocumentFragment failed");
            for x in new {
                match x {
                    Meta::Text{dom_ref} => {
                        fragment.append_child(dom_ref.dom_ref_as_node())
                            .expect("DocumentFragment.append failed");
                    }
                    Meta::Tag{dom_ref} => {
                        fragment.append_child(dom_ref.dom_ref_as_node())
                            .expect("DocumentFragment.append failed");
                    }
                }
            }
            fragment
        }
        match op {
            InsertOp::InsertBefore {old, new} => {
                let new = init_fragment(new);
                let new: wasm_bindgen::JsValue = From::from(new);
                let new: web_sys::Node = web_sys::Node::from(new);
                
                let old: wasm_bindgen::JsValue = old.get_dom_ref().dom_ref().clone();
                let old: web_sys::Element = web_sys::Element::from(old);
                old.before_with_node_1(&new);
            }
            InsertOp::InsertAfter {old, new} => {
                let new = init_fragment(new);
                let new: wasm_bindgen::JsValue = From::from(new);
                let new: web_sys::Node = web_sys::Node::from(new);
                
                let old: wasm_bindgen::JsValue = old.get_dom_ref().dom_ref().clone();
                let old: web_sys::Element = web_sys::Element::from(old);
                old.after_with_node_1(&new);
            }
            InsertOp::Swap {parent, new, old} => {
                parent
                    .get_dom_ref()
                    .replace_child(new.get_dom_ref().as_ref(), old.get_dom_ref().as_ref());
            }
            InsertOp::Append {parent, new} => {
                let new = init_fragment(new);
                let new: wasm_bindgen::JsValue = From::from(new);
                let new: web_sys::Node = web_sys::Node::from(new);
                parent
                    .get_dom_ref()
                    .dom_ref_as_node()
                    .append_child(&new)
                    .expect("Node.appendChild failed");
            }
        }
    }
    fn remove(&self, x: Meta) {
        let x: wasm_bindgen::JsValue = x.get_dom_ref().dom_ref().clone();
        let x: web_sys::Element = web_sys::Element::from(x);
        x.remove();
    }
}












