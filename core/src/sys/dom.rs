use std::fmt::{self, Debug};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::collections::*;
use std::cell::{self, Cell, RefCell};
use std::rc::Rc;
use std::any::Any;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
use either::Either;
use wasm_bindgen::JsValue;
use wasm_bindgen::closure;
use wasm_bindgen::closure::Closure;


///////////////////////////////////////////////////////////////////////////////
// INTERNAL UTILS
///////////////////////////////////////////////////////////////////////////////

pub mod event {
    use super::*;
    
    pub fn get_oninput_value(event: &JsValue) -> String {
        let event: web_sys::Event = From::from(event.clone());
        let target: web_sys::EventTarget = event
            .target()
            .expect("target failed");
        let target: JsValue = From::from(target);
        let target: web_sys::HtmlInputElement = From::from(target);
        let value = target.value();
        value
    }
    pub fn prevent_default(event: &JsValue) {
        let event: web_sys::Event = From::from(event.clone());
        event.prevent_default();
    }
}

pub mod tag {
    pub fn is_svg(tag: &str) -> bool {
        match tag.to_lowercase().as_str() {
            "animate" => true,
            "animatemotion" => true,
            "animatetransform" => true,
            "circle" => true,
            "clippath" => true,
            "defs" => true,
            "desc" => true,
            "discard" => true,
            "ellipse" => true,
            "feblend" => true,
            "fecolormatrix" => true,
            "fecomponenttransfer" => true,
            "fecomposite" => true,
            "feconvolvematrix" => true,
            "fediffuselighting" => true,
            "fedisplacementmap" => true,
            "fedistantlight" => true,
            "fedropshadow" => true,
            "feflood" => true,
            "fefunca" => true,
            "fefuncb" => true,
            "fefuncg" => true,
            "fefuncr" => true,
            "fegaussianblur" => true,
            "feimage" => true,
            "femerge" => true,
            "femergenode" => true,
            "femorphology" => true,
            "feoffset" => true,
            "fepointlight" => true,
            "fespecularlighting" => true,
            "fespotlight" => true,
            "fetile" => true,
            "feturbulence" => true,
            "filter" => true,
            "foreignobject" => true,
            "g" => true,
            "line" => true,
            "lineargradient" => true,
            "marker" => true,
            "mask" => true,
            "metadata" => true,
            "mpath" => true,
            "path" => true,
            "pattern" => true,
            "polygon" => true,
            "polyline" => true,
            "radialgradient" => true,
            "rect" => true,
            "set" => true,
            "stop" => true,
            "svg" => true,
            "switch" => true,
            "symbol" => true,
            "text" => true,
            "textpath" => true,
            "title" => true,
            "tspan" => true,
            "unknown" => true,
            "use" => true,
            "view" => true,
            _ => false,
        }
    }
}

pub mod core {
    pub fn get_window() -> web_sys::Window {
        let window: web_sys::Window = web_sys::window()
            .expect("window not available");
        window
    }
    pub fn get_document() -> web_sys::Document {
        let window: web_sys::Window = web_sys::window()
            .expect("window not available");
        let document = window
            .document()
            .expect("document not available");
        document
    }
    pub fn get_body_as_node() -> web_sys::Node {
        let window: web_sys::Window = web_sys::window()
            .expect("window not available");
        let document = window
            .document()
            .expect("document not available");
        let body: web_sys::Node = std::convert::From::from(
            document.body().expect("document.body not available")
        );
        body
    }
    pub fn get_body() -> web_sys::Element {
        let window: web_sys::Window = web_sys::window()
            .expect("window not available");
        let document = window
            .document()
            .expect("document not available");
        let body: web_sys::Element = std::convert::From::from(
            document.body().expect("document.body not available")
        );
        body
    }
    pub fn new_element(tag: &str) -> web_sys::Element {
        let document = get_document();
        let result = document.create_element(tag)
            .expect("failed to create element");
        result
    }
    pub fn new_svg_element(tag: &str) -> web_sys::Element {
        let document = get_document();
        let result = document.create_element_ns(
                Some("http://www.w3.org/2000/svg"),
                tag,
            )
            .expect("failed to create element");
        result
    }
    pub fn new_text(value: &str) -> web_sys::Text {
        let document = get_document();
        let result = document.create_text_node(value);
        result
    }
}

///////////////////////////////////////////////////////////////////////////////
// GENERIC EVENTS
///////////////////////////////////////////////////////////////////////////////

pub trait Callback {
    fn as_js_function(&self) -> &js_sys::Function;
}


///////////////////////////////////////////////////////////////////////////////
// GENERIC DOM TREE API
///////////////////////////////////////////////////////////////////////////////

pub trait DomRef {
    fn dom_ref(&self) -> &JsValue;
}

pub trait DomNode: DomRef {
    fn dom_ref_as_node(&self) -> &web_sys::Node;
    fn dom_ref_as_element(&self) -> &web_sys::Element;
    
    fn add_event_listener(&self, event_name: &str, callback: &js_sys::Function) {
        self.dom_ref_as_node().add_event_listener_with_callback(event_name, &callback)
            .expect("addEventListener failed");
    }
    fn remove_event_listener(&self, event_name: &str, callback: &js_sys::Function) {
        self.dom_ref_as_node().remove_event_listener_with_callback(event_name, &callback)
            .expect("removeEventListener failed");
    }
    fn set_attribute(&self, key: &str, value: &str) {
        self.dom_ref_as_element().set_attribute(key, value)
            .expect("setAttribute failed");
    }
    fn remove_attribute(&self, key: &str) {
        self.dom_ref_as_element().remove_attribute(key)
            .expect("removeAttribute failed");
    }
    fn append_child(&self, child: &DomNode) {
        self.dom_ref_as_node()
            .append_child(&child.dom_ref_as_node())
            .expect("appendChild failed");
    }
    fn remove_child(&self, child: &DomNode) {
        self.dom_ref_as_node()
            .remove_child(&child.dom_ref_as_node())
            .expect("removeChild failed");
    }
    fn replace_child(&self, new_child: &DomNode, old_child: &DomNode) {
        self.dom_ref_as_node()
            .replace_child(&new_child.dom_ref_as_node(), &old_child.dom_ref_as_node())
            .expect("replacedNode failed");
    }
}


///////////////////////////////////////////////////////////////////////////////
// WINDOW
///////////////////////////////////////////////////////////////////////////////

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

cached!{
    WINDOW;
    fn window() -> Window = {
        Window::new()
    }
}


#[derive(Clone, Debug)]
pub struct Window {
    pub instance: web_sys::Window,
    pub local_storage: Storage,
    pub document: Document,
    pub location: Location,
    pub history: History,
}

impl Window {
    pub fn new() -> Self {
        let window = core::get_window();
        Window {
            instance: window,
            local_storage: Storage::new(),
            document: Document::new(),
            location: Location::new(),
            history: History::new(),
        }
    }
    pub fn device_pixel_ratio(&self) -> f64 {
        self.instance.device_pixel_ratio()
    }
    pub fn request_animation_frame(&self, cb: &Callback) {
        self.instance.request_animation_frame(cb.as_js_function());
    }
    pub fn set_timeout(&self, cb: &Callback, timeout: i32) {
        self.instance.set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_js_function(),
            timeout
        );
    }
}


///////////////////////////////////////////////////////////////////////////////
// LOCATION
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Location {
    instance: web_sys::Location
}

impl Location {
    pub fn new() -> Self {
        Location {
            instance: core::get_window()
                .location()
        }
    }
    pub fn pathname(&self) -> String {
        self.instance
            .pathname()
            .expect("pathname failed")
    }
}



///////////////////////////////////////////////////////////////////////////////
// HISTORY
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct History {
    instance: web_sys::History
}

impl History {
    pub fn new() -> Self {
        History {
            instance: core::get_window()
                .history()
                .expect("window.history getter failed"),
        }
    }
    pub fn push_state(&self, url_path: &str) {
        self.instance.push_state_with_url(
            &JsValue::null(),
            "",
            Some(url_path)
        )
        .expect("pushState failed");
    }
}


///////////////////////////////////////////////////////////////////////////////
// STORAGE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Storage {
    instance: web_sys::Storage
}

impl Storage {
    pub fn new() -> Self {
        let instance = core::get_window()
            .local_storage()
            .expect("localStorage failed")
            .expect("localStorage missing");
        Storage {
            instance: instance,
        }
    }
    pub fn get<Value>(&self, key: &str) -> Option<Value>
    where
        Value: DeserializeOwned
    {
        let value = self.instance
            .get_item(key)
            .expect("getItem method failed");
        match value {
            None => None,
            Some(value) => match serde_json::from_str(value.clone().as_str()) {
                Err(msg) => None,
                Ok(value) => Some(value)
            }
        }
    }
    pub fn set<Value: Serialize>(&self, key: &str, value: &Value) {
        match serde_json::to_string(value) {
            Err(msg) => (),
            Ok(value) => self.instance
                .set_item(key, value.as_str())
                .expect("setItem method failed")
        }
    }
    pub fn remove(&self, key: &str) {
        self.instance
            .remove_item(key)
            .expect("removeItem method failed")
    }
}

///////////////////////////////////////////////////////////////////////////////
// DOCUMENT
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Document {
    body: Body
}

impl Document {
    pub fn new() -> Self {
        Document {
            body: Body::new()
        }
    }
    pub fn crate_text_node(&self, initial_value: &str) -> Text {
        let dom_ref_as_text: web_sys::Text = core::new_text(initial_value);
        let dom_ref: JsValue = From::from(dom_ref_as_text.clone());
        let dom_ref_as_node: web_sys::Node = From::from(dom_ref.clone());
        Text {dom_ref_as_text, dom_ref, dom_ref_as_node}
    }
    pub fn crate_element(&self, tag: &str) -> Tag {
        let element = {
            if tag::is_svg(tag) {
                core::new_svg_element(tag)
            } else {
                core::new_svg_element(tag)
            }
        };
        let dom_ref: JsValue = From::from(element.clone());
        Tag {
            tag: String::from(tag),
            dom_ref_as_node: From::from(dom_ref.clone()),
            dom_ref,
            dom_ref_as_element: element,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// HEAD
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Head {
    
}



///////////////////////////////////////////////////////////////////////////////
// BODY
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Body {
    
}

impl Body {
    pub fn new() -> Self {
        Body {}
    }
}


///////////////////////////////////////////////////////////////////////////////
// TAG
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Tag {
    pub tag: String,
    pub dom_ref: JsValue,
    pub dom_ref_as_node: web_sys::Node,
    pub dom_ref_as_element: web_sys::Element,
}

impl Tag {
    pub fn new(tag: &str) -> Self {
        let element = {
            if tag::is_svg(tag) {
                core::new_svg_element(tag)
            } else {
                core::new_svg_element(tag)
            }
        };
        let dom_ref: JsValue = From::from(element.clone());
        Tag {
            tag: String::from(tag),
            dom_ref_as_node: From::from(dom_ref.clone()),
            dom_ref: dom_ref,
            dom_ref_as_element: element,
        }
    }
}

impl DomRef for Tag {
    fn dom_ref(&self) -> &JsValue {
        &self.dom_ref
    }
}

impl DomNode for Tag {
    fn dom_ref_as_node(&self) -> &web_sys::Node {
        &self.dom_ref_as_node
    }
    fn dom_ref_as_element(&self) -> &web_sys::Element {
        &self.dom_ref_as_element
    }
}



///////////////////////////////////////////////////////////////////////////////
// TEXT
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Text {
    pub dom_ref: JsValue,
    pub dom_ref_as_text: web_sys::Text,
    pub dom_ref_as_node: web_sys::Node,
}

impl Text {
    pub fn new(initial_value: &str) -> Self {
        let dom_ref_as_text: web_sys::Text = core::new_text(initial_value);
        let dom_ref: JsValue = From::from(dom_ref_as_text.clone());
        let dom_ref_as_node: web_sys::Node = From::from(dom_ref.clone());
        Text {
            dom_ref_as_text: dom_ref_as_text,
            dom_ref: dom_ref,
            dom_ref_as_node: dom_ref_as_node,
        }
    }
    pub fn set_text_content(&self, value: &str) {
        self.dom_ref_as_node.set_text_content(Some(value));
    }
}

impl DomRef for Text {
    fn dom_ref(&self) -> &JsValue {
        &self.dom_ref
    }
}

