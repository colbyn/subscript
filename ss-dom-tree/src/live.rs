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

pub type LiveTree<Msg> = ITree<LiveNode<Msg>, LiveLeaf>;


///////////////////////////////////////////////////////////////////////////////
// LIVE DOM-REF
///////////////////////////////////////////////////////////////////////////////

pub enum DomRef {
    Text(dom::Text),
    Tag(dom::Tag),
}


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


pub struct LiveNode<Msg: PartialEq> {
    pub dom_ref: Rc<dom::Tag>,
    pub tag: String,
    pub attributes: RefCell<IMap<String, attributes::Attribute>>,
    pub events: RefCell<IMap<events::EventType, LiveEventHandler<Msg>>>,
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

// use insertion_types::tree::map::*;

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







