#![allow(dead_code, unused, unused_variables)]


use std::fmt::Debug;
use std::cell::*;
use std::convert::From;
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
use ss_css_types::api::Stylesheet;
use ss_cssom_tree::GLOBAL_CSS_REGISTRY;


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
                window.document.body.append_child(&mount);
                Rc::new(mount)
            },
        };
        let tree = STree::from(
            &sync_api,
            &mount,
            initial_view.0
        );
        LiveView {sync_api,mount,tree}
    }
    pub fn sync(&mut self, view: View<Msg>) {
        self.tree.traverse_pair(&self.sync_api, &view.0, &PairTraversal {
            leafs: &move |n1, n2| {},
            nodes: &move |n1, n2| {
                n1.events.borrow().traverse_values_pair(&n2.events, &move |e1, e2| {
                    e1.sync(e2);
                });
            },
        });
        self.tree.sync(&self.sync_api, &self.mount, view.0);
    }
    pub fn tick(&self, env: &mut TickEnv<Msg>, reg: &GlobalTickRegistry) {
        let mut nf = |node: &LiveNode<Msg>| -> () {
            node.events.borrow_mut().traverse_values_mut(|handler| {
                let f: &EventHandler<Msg> = &handler.value.borrow();
                env.messages.append(
                    &mut handler.callback.drain()
                        .into_iter()
                        .map(|event| f.run_handler(event))
                        .collect::<Vec<Msg>>()
                );
            });
        };
        let mut lf = |leaf: &LiveLeaf| -> () {
            match leaf {
                LiveLeaf::Text{value, ..} => {}
                LiveLeaf::Component{value, ..} => {
                    reg.components.borrow_mut().push(value.tick(reg));
                }
            }
        };
        self.tree.traverse(&mut nf, &mut lf);
    }
}




///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////

pub type AttributesMap<Msg> = SMap<LiveNode<Msg>, String, AttributeValue, AttributeValue>;

#[derive(Debug, PartialEq)]
pub struct AttributesApi {}

impl<Msg> MapApi<LiveNode<Msg>, String, AttributeValue, AttributeValue> for AttributesApi
where
    Msg: PartialEq + 'static + Debug + Clone
{
    fn create(&self, attached: &LiveNode<Msg>, key: &String, new: AttributeValue) -> AttributeValue {
    	match &new {
            AttributeValue::Value(str) => attached.dom_ref.set_attribute(key, str),
            AttributeValue::Toggle(true) => attached.dom_ref.set_attribute(key, ""),
            AttributeValue::Toggle(false) => (),
        }
        new
    }
    fn modified(&self, attached: &LiveNode<Msg>, key: &String, old: &mut AttributeValue, new: AttributeValue) {
        // SPECIAL - SYNC STATEFUL ATTRIBUTE CONTROLLED DOM VALUES
        match (attached.tag.as_str(), key.as_str()) {
            ("input", "value") => {
                if let Some(value) = new.get_string() {
                    set_input_value(attached.dom_ref.as_ref(), value);
                }
            },
            _ => ()
        }
        // UPDATE ATTRIBUTE
        match &new {
            AttributeValue::Value(str) => attached.dom_ref.set_attribute(key, str),
            AttributeValue::Toggle(true) => attached.dom_ref.set_attribute(key, ""),
            AttributeValue::Toggle(false) => attached.dom_ref.remove_attribute(key),
        }
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
fn set_input_value(dom_ref: &dom::Tag, value: &str) {
    let node_ref: JsValue = From::from(dom_ref.dom_ref_as_element());
    let node_ref: web_sys::HtmlInputElement = From::from(node_ref);
    node_ref.set_value(value);
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

#[derive(Debug, PartialEq)]
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


#[derive(Debug)]
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

#[derive(Debug)]
pub struct LiveNode<Msg> where Msg: PartialEq + Clone + Debug {
    pub auto_listeners: HashMap<String, VoidCallback>,
    pub dom_ref: Rc<dom::Tag>,
    pub tag: String,
    pub attributes: RefCell<AttributesMap<Msg>>,
    pub events: RefCell<EventsMap<Msg>>,
    pub styling: RefCell<Stylesheet>,
}

impl<Msg> Drop for LiveNode<Msg> where Msg: PartialEq + Clone + Debug {
    fn drop(&mut self) {
        for (event_name, callback) in self.auto_listeners.drain() {
            self.dom_ref.remove_event_listener(&event_name, &callback);
        }
        for (key, live_event_listener) in self.events.borrow_mut().dangerous_unsync_drain() {
            self.dom_ref.remove_event_listener(key.as_str(), &live_event_listener.callback);
        }
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

#[derive(Debug)]
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
        let attributes_unchanged = old.attributes.borrow().unchanged(
            &self.attributes_api,
            &new.attributes,
        );
        let events_unchanged = {
            let mut ks = Vec::new();
            for k in new.events.keys() {
                ks.push(k.clone());
            }
            old.events.borrow().get_keys() == ks
        };
        let styling_unchanged = *old.styling.borrow() == new.styling;
        let result = styling_unchanged && attributes_unchanged && events_unchanged && new.tag == old.tag;
        result
    }
    fn node_recyclable(&self, new: &ViewNode<Msg>, old: &LiveNode<Msg>) -> bool {
        let events_recyclable = {
            let mut ks = Vec::new();
            for k in new.events.keys() {
                ks.push(k.clone());
            }
            old.events.borrow().get_keys() == ks
        };
        (new.tag == old.tag) && events_recyclable
    }
    fn node_update(&self, update: Update<&mut LiveNode<Msg>, ViewNode<Msg>>) {
        use ss_web_utils::dom::DomRef;
        let Update{new, old} = update;
        assert!(new.tag == old.tag);
        old.attributes.borrow_mut().sync(
            &self.attributes_api,
            &old,
            new.attributes,
        );
        old.events.borrow_mut().sync(
            &self.events_api,
            &old,
            new.events,
        );
        let styling_unchanged = {*old.styling.borrow() == new.styling};
        if !styling_unchanged {
            let hash_key: u64 = GLOBAL_CSS_REGISTRY.with({
                let sheet = new.styling.clone();
                |reg| reg.borrow_mut().upsert(sheet)
            });
            old.styling.replace(new.styling);
            old.dom_ref.set_attribute("css", &format!("{}", &hash_key));
        }
    }
    fn node_crate(&self, new: ViewNode<Msg>) -> LiveNode<Msg> {
        use ss_web_utils::dom::DomRef;
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
        let result = LiveNode {
            auto_listeners,
            styling: RefCell::new(new.styling),
            dom_ref: Rc::new(dom_ref),
            tag: new.tag,
            attributes: RefCell::new(SMap::default()),
            events: RefCell::new(SMap::default()),
        };
        result.attributes.borrow_mut().sync(
            &self.attributes_api,
            &result,
            new.attributes,
        );
        result.events.borrow_mut().sync(
            &self.events_api,
            &result,
            new.events,
        );
        let styling_empty = result.styling.borrow().is_empty();
        if !styling_empty {
            let hash_key: u64 = GLOBAL_CSS_REGISTRY.with({
                let sheet = result.styling.borrow().clone();
                |reg| reg.borrow_mut().upsert(sheet)
            });
            result.dom_ref.set_attribute("css", &format!("{}", &hash_key));
        }
        result
    }
    fn leaf_unchanged(&self, new: &ViewLeaf, old: &LiveLeaf) -> bool {
        match (new, old) {
            (ViewLeaf::Component(new), LiveLeaf::Component{value: old, ..}) => {
                let new: Box<Any> = Box::new(new.clone());
                old.unchanged(&new)
            },
            (ViewLeaf::Text(x), LiveLeaf::Text{value, ..}) => x == value,
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
    fn leaf_update(&self, update: Update<&mut LiveLeaf, ViewLeaf>) {
        let Update{new, old} = update;
        match (&new, old) {
            (ViewLeaf::Text(x), LiveLeaf::Text{value, dom_ref}) => {
                dom_ref.set_text_content(value.as_str());
                *value = x.clone();
            }
            (ViewLeaf::Component(_), LiveLeaf::Component{value, dom_ref}) => {
                panic!()
            }
            _ => panic!()
        }
    }
    fn leaf_crate(&self, new: ViewLeaf) -> LiveLeaf {
        use ss_web_utils::dom::DomRef;
        match new {
            ViewLeaf::Text(value) => {
                let dom_ref = self.window.document.create_text_node(value.as_str());
                dom_ref.set_text_content(value.as_str());
                LiveLeaf::Text {
                    dom_ref: Rc::new(dom_ref),
                    value,
                }
            }
            ViewLeaf::Component(value) => {
                let dom_ref = self.window.document.create_element("div");
                LiveLeaf::Component {
                    dom_ref: Rc::new(dom_ref),
                    value
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
            InsertOp::Swap {parent, current, target} => {
                parent
                    .get_dom_ref()
                    .replace_child(target.get_dom_ref().as_ref(), current.get_dom_ref().as_ref());
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












