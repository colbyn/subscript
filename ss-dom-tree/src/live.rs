use std::cell::*;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::rc::Rc;
use std::any::*;
use std::collections::*;
use either::Either::{self, Left, Right};

use web_utils::prelude::*;
use web_utils::dom;
use web_utils::js::{self, console, EventCallback};
use ss_trees::tree::*;
use ss_trees::map::*;

use crate::html::*;
use crate::html::events::*;
use crate::html::attributes::*;


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type LiveTree<Msg> = STree<Meta, LiveNode<Msg>, LiveLeaf, ViewNode<Msg>, ViewLeaf>;


///////////////////////////////////////////////////////////////////////////////
// ATTRIBUTES
///////////////////////////////////////////////////////////////////////////////

pub struct AttributesLogic {}

impl<Msg: PartialEq> IMapLogic<LiveNode<Msg>, String, Attribute, Attribute> for AttributesLogic {
    fn for_added(&self, attached: &LiveNode<Msg>, key: &String, new: Attribute) -> Attribute {
    	match &new {
            Attribute::Value(str) => attached.dom_ref.set_attribute(key, str),
            Attribute::Toggle(true) => attached.dom_ref.set_attribute(key, ""),
            Attribute::Toggle(false) => (),
        }
        new
    }
    fn for_modified(&self, attached: &LiveNode<Msg>, key: &String, old: &mut Attribute, new: Attribute) {
    	attached.dom_ref.remove_attribute(key);
        match &new {
            Attribute::Value(str) => attached.dom_ref.set_attribute(key, str),
            Attribute::Toggle(true) => attached.dom_ref.set_attribute(key, ""),
            Attribute::Toggle(false) => (),
        }
        *old = new;
    }
    fn for_removed(&self, attached: &LiveNode<Msg>, key: String, old: Attribute) {
    	attached.dom_ref.remove_attribute(&key);
    }
    fn is_unchanged(&self, old: &Attribute, new: &Attribute) -> bool {
    	old == new
    }
}




///////////////////////////////////////////////////////////////////////////////
// EVENTS
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct LiveEventHandler<Msg> {
    pub value: Rc<EventHandler<Msg>>,
    pub callback: js::EventCallback<Msg>,
}

impl<Msg> LiveEventHandler<Msg> {
    /// Gets the DOM event name.
    pub fn event_name(&self) -> EventType {
        self.value.event_name()
    }
}
impl<Msg> js::Handler<Msg> for LiveEventHandler<Msg> {
    fn handler(&self, event: wasm_bindgen::JsValue) -> Msg {
        self.value.handler(event)
    }
}
impl<Msg> dom::Callback for LiveEventHandler<Msg> {
    fn as_js_function(&self) -> &js_sys::Function {
        self.callback.as_js_function()
    }
}


pub struct EventsLogic {}

impl<Msg: PartialEq + 'static> IMapLogic<LiveNode<Msg>, EventType, EventHandler<Msg>, LiveEventHandler<Msg>> for EventsLogic {
    fn for_added(&self, attached: &LiveNode<Msg>, key: &EventType, new: EventHandler<Msg>) -> LiveEventHandler<Msg> {
    	use web_utils::js::Handler;
        use web_utils::dom::DomRef;

        assert!({key == &new.event_name()});
        let x = dom::window();
        let value = Rc::new(new);
        let callback = js::EventCallback::new(value.clone());
        attached.dom_ref.add_event_listener(key.as_str(), &callback);
        LiveEventHandler {
            callback,
            value,
        }
    }
    fn for_modified(&self, attached: &LiveNode<Msg>, key: &EventType, old: &mut LiveEventHandler<Msg>, new: EventHandler<Msg>) {
    	use web_utils::js::Handler;
        use web_utils::dom::DomRef;

        assert!(key == &old.event_name() && key == &new.event_name());
        attached.dom_ref.remove_event_listener(key.as_str(), &old.callback);
        let value = Rc::new(new);
        let callback = js::EventCallback::new(value.clone());
        attached.dom_ref.add_event_listener(key.as_str(), &callback);
        let result = LiveEventHandler {
            callback,
            value,
        };
        *old = result;
    }
    fn for_removed(&self, attached: &LiveNode<Msg>, key: EventType, old: LiveEventHandler<Msg>) {
        use web_utils::dom::DomRef;

    	assert_eq!(key, old.event_name());
        attached.dom_ref.remove_event_listener(key.as_str(), &old.callback);
    }
    fn is_unchanged(&self, old: &LiveEventHandler<Msg>, new: &EventHandler<Msg>) -> bool {
    	old.value.as_ref() == new
    }
}



///////////////////////////////////////////////////////////////////////////////
// LIVE COMPONENT
///////////////////////////////////////////////////////////////////////////////

pub type ProcessId = String;

pub trait LiveComponent {
    fn spec_type_id(&self) -> TypeId;
    fn process_id(&self) -> ProcessId;
    fn dom_ref(&self) -> &DomRef;
    fn tick(&self, sub_enqueue: &Vec<Rc<Any>>);
    fn box_clone(&self) -> Box<LiveComponent>;
}

impl Clone for Box<LiveComponent> {
    fn clone(&self) -> Box<LiveComponent> {
        self.box_clone()
    }
}
impl PartialEq for LiveComponent {
    fn eq(&self, other: &LiveComponent) -> bool {
        self.spec_type_id() == other.spec_type_id()
    }
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
        value: Box<Component>,
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
pub struct LiveNode<Msg: PartialEq> {
    pub dom_ref: Rc<dom::Tag>,
    pub tag: String,
    pub attributes: RefCell<IMap<String, attributes::Attribute>>,
    pub events: RefCell<IMap<events::EventType, LiveEventHandler<Msg>>>,
}

impl<Msg: PartialEq> LiveNode<Msg> {
    pub fn get_meta(&self) -> Meta {
        let dom_ref = self.dom_ref.clone();
        Meta::Tag{dom_ref}
    }
}

impl<Msg: PartialEq> PartialEq for LiveNode<Msg> {
    fn eq(&self, other: &LiveNode<Msg>) -> bool {
        self.tag == other.tag &&
        self.attributes == other.attributes &&
        self.events == other.events
    }
}


///////////////////////////////////////////////////////////////////////////////
// LOGIC
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

pub struct DomTreeLogic {
    pub window: dom::Window,
    pub attributes_api: AttributesLogic,
    pub events_api: EventsLogic,
}

impl Default for DomTreeLogic {
    fn default() -> Self {
        DomTreeLogic {
            window: dom::window(),
            attributes_api: AttributesLogic {},
            events_api: EventsLogic {},
        }
    }
}

impl<Msg: 'static +  Clone + PartialEq> TreeApi<Meta, LiveNode<Msg>, LiveLeaf, ViewNode<Msg>, ViewLeaf> for DomTreeLogic {
    fn node_unchanged(&self, new: &ViewNode<Msg>, old: &LiveNode<Msg>) -> bool {
        let attributes_unchanged = old.attributes.borrow().unchanged::<LiveNode<Msg>, Attribute>(&new.attributes, &self.attributes_api);
        let events_unchanged = old.events.borrow().unchanged(&new.events, &self.events_api);
        attributes_unchanged && events_unchanged && new.tag == old.tag
    }
    fn node_recyclable(&self, new: &ViewNode<Msg>, old: &LiveNode<Msg>) -> bool {
        new.tag == old.tag
    }
    fn node_update(&self, update: Update<&mut LiveNode<Msg>, ViewNode<Msg>>) {
        use web_utils::dom::DomRef;
        let Update{new, old} = update;
        assert!(new.tag == old.tag);
        old.attributes.borrow_mut().sync::<LiveNode<Msg>, Attribute>(&old, new.attributes, &self.attributes_api);
        old.events.borrow_mut().sync::<LiveNode<Msg>, EventHandler<Msg>>(&old, new.events, &self.events_api);
    }
    fn node_crate(&self, new: ViewNode<Msg>) -> LiveNode<Msg> {
        use web_utils::dom::DomRef;
        let dom_ref = self.window.document.create_element(new.tag.as_str());
        LiveNode {
            dom_ref: Rc::new(dom_ref),
            tag: new.tag,
            attributes: RefCell::new(IMap::new()),
            events: RefCell::new(IMap::new()),
        }
    }

    fn leaf_unchanged(&self, new: &ViewLeaf, old: &LiveLeaf) -> bool {
        match (new, old) {
            (ViewLeaf::Component(x), LiveLeaf::Component{value, ..}) => {
                x.spec_type_id() == value.spec_type_id()
            },
            (ViewLeaf::Text(x), LiveLeaf::Text{value, ..}) => x == value,
            _ => false
        }
    }
    fn leaf_recyclable(&self, new: &ViewLeaf, old: &LiveLeaf) -> bool {
        match (new, old) {
            (ViewLeaf::Text(_), LiveLeaf::Text{..}) => true,
            (ViewLeaf::Component(_), LiveLeaf::Component{..}) => false,
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
        use web_utils::dom::DomRef;
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
                        // fragment.append_with_node_1(dom_ref.dom_ref_as_node())
                        //     .expect("DocumentFragment.append failed");
                        fragment.append_child(dom_ref.dom_ref_as_node())
                            .expect("DocumentFragment.append failed");
                    }
                    Meta::Tag{dom_ref} => {
                        // fragment.append_with_node_1(dom_ref.dom_ref_as_node())
                        //     .expect("DocumentFragment.append failed");
                        fragment.append_child(dom_ref.dom_ref_as_node())
                            .expect("DocumentFragment.append failed");
                    }
                }
            }
            fragment
        }
        match op {
            InsertOp::InsertBefore {old, new} => {
                console::log(format!("dom-tree: {:#?}", (&old, &new)));
                let new = init_fragment(new);
                let new: wasm_bindgen::JsValue = From::from(new);
                let new: web_sys::Node = web_sys::Node::from(new);
                
                let old: wasm_bindgen::JsValue = old.get_dom_ref().dom_ref().clone();
                let old: web_sys::Element = web_sys::Element::from(old);
                old.before_with_node_1(&new);
            }
            InsertOp::InsertAfter {old, new} => {
                console::log(format!("dom-tree: {:#?}", (&old, &new)));
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








