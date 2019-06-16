pub mod extra;

use core::fmt::Debug;
use std::fmt;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsValue, JsCast};
use js_sys::Function;

use crate::{console};
use crate::backend::css;


///////////////////////////////////////////////////////////////////////////////
// COMMON DOM APIs
///////////////////////////////////////////////////////////////////////////////

pub trait EventListenerApi {
	fn as_js_function(&self) -> &Function;
}
pub trait NodeApi {
	fn box_clone(&self) -> Box<NodeApi>;
	fn dom_ref(&self) -> JsValue;
	fn dom_ref_as_node(&self) -> web_sys::Node {
		let value: web_sys::Node = From::from(self.dom_ref());
		value
	}
	fn add_event_listener(&self, event_name: &str, callback: &EventListenerApi) {
	    self.dom_ref_as_node()
	    	.add_event_listener_with_callback(event_name, callback.as_js_function())
	        .expect("addEventListener failed");
	}
	fn remove_event_listener(&self, event_name: &str, callback: &EventListenerApi) {
	    self.dom_ref_as_node()
	    	.remove_event_listener_with_callback(event_name, callback.as_js_function())
	        .expect("removeEventListener failed");
	}
	fn append_child(&self, child: &NodeApi) {
	    self.dom_ref_as_node()
	        .append_child(&child.dom_ref_as_node())
	        .expect("appendChild failed");
	}
	fn remove_child(&self, child: &NodeApi) {
	    self.dom_ref_as_node()
	        .remove_child(&child.dom_ref_as_node())
	        .expect("removeChild failed");
	}
	fn replace_child(&self, new_child: &NodeApi, old_child: &NodeApi) {
	    self.dom_ref_as_node()
	        .replace_child(&new_child.dom_ref_as_node(), &old_child.dom_ref_as_node())
	        .expect("replacedNode failed");
	}
	fn insert_before(&self, new_child: &NodeApi, ref_child: &NodeApi) {
	    self.dom_ref_as_node()
	        .insert_before(
	            &new_child.dom_ref_as_node(),
	            Some(&ref_child.dom_ref_as_node()),
	        )
	        .expect("replacedNode failed");
	}
}

pub trait ElementApi: NodeApi {
	fn dom_ref_as_element(&self) -> web_sys::Element {
		let value: web_sys::Element = From::from(self.dom_ref());
		value
	}
	fn set_attribute(&self, key: &str, value: &str) {
	    self.dom_ref_as_element()
	    	.set_attribute(key, value)
	        .expect("setAttribute failed");
	}
	fn remove_attribute(&self, key: &str) {
	    self.dom_ref_as_element()
	    	.remove_attribute(key)
	        .expect("removeAttribute failed");
	}
	fn insert_adjacent_element(&self, position: AdjacentPosition, element: &ElementApi) {
		self.dom_ref_as_element()
			.insert_adjacent_element(
				position.as_str(),
				&element.dom_ref_as_element(),
			)
			.expect("ElementApi.insert_adjacent_element failed");
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AdjacentPosition {
	/// Before the targetElement
	BeforeBegin,
	/// Inside the targetElement; before its <b>first child</b>.
	AfterBegin,
	/// Inside the targetElement; after its <b>last child</b>.
	BeforeEnd,
	/// After the targetElement.
	AfterEnd,
}

impl AdjacentPosition {
	pub fn as_str(&self) -> &str {
		match self {
			AdjacentPosition::BeforeBegin => "beforebegin",
			AdjacentPosition::AfterBegin => "afterbegin",
			AdjacentPosition::BeforeEnd => "beforeend",
			AdjacentPosition::AfterEnd => "afterend",
		}
	}
}


///////////////////////////////////////////////////////////////////////////////
// DOM NODES
///////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct Body {
	pub instance: JsValue,
}
#[derive(Clone, Debug)]
pub struct Text {
	pub instance: JsValue,
}
#[derive(Clone, Debug)]
pub struct Element {
	pub instance: JsValue,
	pub class_list: ClassList,
}
#[derive(Clone, Debug)]
pub struct ClassList {
	pub instance: JsValue,
}

impl NodeApi for Text {
	fn box_clone(&self) -> Box<NodeApi> {Box::new(self.clone())}
	fn dom_ref(&self) -> JsValue {self.instance.clone()}
}
impl NodeApi for Element {
	fn box_clone(&self) -> Box<NodeApi> {Box::new(self.clone())}
	fn dom_ref(&self) -> JsValue {self.instance.clone()}
}
impl NodeApi for Body {
	fn box_clone(&self) -> Box<NodeApi> {Box::new(self.clone())}
	fn dom_ref(&self) -> JsValue {self.instance.clone()}
}

impl ElementApi for Body {}
impl ElementApi for Element {}

impl Text {
	pub fn dom_ref_as_text(&self) -> web_sys::Text {
		From::from(self.instance.clone())
	}
	pub fn set_text_content(&self, new_value: &str) {
		self.dom_ref_as_text()
			.set_text_content(Some(new_value));
	}
}

impl ClassList {
	pub fn dom_ref_as_dom_token_list(&self) -> web_sys::DomTokenList {
		From::from(self.instance.clone())
	}
	pub fn add(&self, class: &str) {
		let interface = self.dom_ref_as_dom_token_list();
		interface
			.add_1(class)
			.expect("ClassList.add() method failed");
	}
	pub fn remove(&self, class: &str) {
		let interface = self.dom_ref_as_dom_token_list();
		interface
			.remove_1(class)
			.expect("ClassList.remove() method failed");
	}
    pub fn replace(&self, old: &str, new: &str) {
        let interface = self.dom_ref_as_dom_token_list();
        interface
            .replace(old, new)
            .expect("ClassList.replace() method failed");
    }
}

impl Element {
	pub fn dom_ref_as_html_style_element(&self) -> web_sys::HtmlStyleElement {
		From::from(self.instance.clone())
	}
}


///////////////////////////////////////////////////////////////////////////////
// CSSOM
///////////////////////////////////////////////////////////////////////////////

pub struct Stylesheet {
	instance: Element,
}

impl Stylesheet {
	pub fn dom_ref_as_css_style_sheet(&self) -> web_sys::CssStyleSheet {
		let interface = self.instance.dom_ref_as_html_style_element();
		let sheet: web_sys::StyleSheet = interface
			.sheet()
			.expect("sheet getter failed");
		let sheet: JsValue = From::from(sheet);
		let sheet: web_sys::CssStyleSheet = From::from(sheet);
		sheet
	}

	pub fn from_element(element: Element) -> Self {
		Stylesheet {instance: element}
	}
	
	pub fn push_declaration(&self, value: css::Declaration) {
		let interface = self.dom_ref_as_css_style_sheet();
		interface
			.insert_rule(&value.as_str())
			.expect("insertRule() method failed");
	}
	pub fn push_keyframes(&self, value: css::Keyframes) {
		let interface = self.dom_ref_as_css_style_sheet();
		interface
			.insert_rule(&value.as_str())
			.expect("insertRule() method failed");
	}
	pub fn push_media(&self, value: css::Media) {
		let interface = self.dom_ref_as_css_style_sheet();
		interface
			.insert_rule(&value.as_str())
			.expect("insertRule() method failed");
	}
}






///////////////////////////////////////////////////////////////////////////////
// CALLBACKS - MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct CallbackSettings {
    pub stop_propagation: bool,
    pub prevent_default: bool,
}

impl Default for CallbackSettings {
	fn default() -> Self {
		let stop_propagation = false;
		let prevent_default = false;
		CallbackSettings{stop_propagation,prevent_default}
	}
}

fn callback_settings_handler(settings: CallbackSettings, value: &JsValue) {
    let event: web_sys::Event = From::from(value.clone());
    if settings.prevent_default {
        event.prevent_default();
    }
    if settings.stop_propagation {
        event.stop_propagation();
    }
}

impl EventListenerApi for js_sys::Function {
    fn as_js_function(&self) -> &Function {
    	self
    }
}

///////////////////////////////////////////////////////////////////////////////
// QUEUE-CALLBACK
///////////////////////////////////////////////////////////////////////////////

pub struct QueueCallback {
    settings: CallbackSettings,
    bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
    events: Rc<RefCell<VecDeque<JsValue>>>,
}
impl QueueCallback {
    pub fn new(dom_ref: &NodeApi, event_type: &str, settings: CallbackSettings) -> Self {
        let events_queue: Rc<RefCell<VecDeque<JsValue>>> = Rc::new(RefCell::new(VecDeque::new()));
        let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let events_queue = events_queue.clone();
            let settings = settings.clone();
            move |value: JsValue| {
                callback_settings_handler(settings.clone(), &value);
                events_queue.borrow_mut().push_back(value);
            }
        }));
        let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
        dom_ref.add_event_listener(event_type, js_function);
        QueueCallback {settings, bindgen_closure: Rc::new(bindgen_closure),events: events_queue}
    }
    pub fn drain(&self) -> Vec<JsValue> {
        self.events.borrow_mut().drain(..).collect()
    }
}
impl std::fmt::Debug for QueueCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "QueueCallback")
    }
}
impl PartialEq for QueueCallback {
    fn eq(&self, other: &QueueCallback) -> bool {true}
}
impl EventListenerApi for QueueCallback {
    fn as_js_function(&self) -> &Function {
    	use wasm_bindgen::JsCast;
    	let bindgen_closure: &Closure<dyn Fn(JsValue)> = &self.bindgen_closure;
    	bindgen_closure.as_ref().unchecked_ref()
    }
}



///////////////////////////////////////////////////////////////////////////////
// VOID-CALLBACK
///////////////////////////////////////////////////////////////////////////////

pub struct VoidCallback {
    settings: CallbackSettings,
    callback: Option<Rc<Fn(JsValue)>>,
    bindgen_closure: Rc<Closure<dyn Fn(JsValue)>>,
}
impl VoidCallback {
	pub fn new(dom_ref: &NodeApi, event_type: &str, settings: CallbackSettings) -> Self {
	    let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
	        let settings = settings.clone();
	        move |value: JsValue| {
	            callback_settings_handler(settings.clone(), &value);
	        }
	    }));
	    let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
	    dom_ref.add_event_listener(event_type, js_function);
	    VoidCallback {settings, callback: None, bindgen_closure: Rc::new(bindgen_closure)}
	}
    pub fn new_with_fn(dom_ref: &NodeApi, event_type: &str, settings: CallbackSettings, callback: impl Fn(JsValue) + 'static) -> Self {
        let callback = Rc::new(callback);
        let bindgen_closure: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let callback = callback.clone();
            let settings = settings.clone();
            move |value: JsValue| {
                callback_settings_handler(settings.clone(), &value);
                callback(value);
            }
        }));
        let js_function: &js_sys::Function = bindgen_closure.as_ref().unchecked_ref();
        dom_ref.add_event_listener(event_type, js_function);
        VoidCallback {settings, callback: Some(callback), bindgen_closure: Rc::new(bindgen_closure)}
    }
}
impl std::fmt::Debug for VoidCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VoidCallback")
    }
}
impl PartialEq for VoidCallback {
    fn eq(&self, other: &VoidCallback) -> bool {true}
}
impl EventListenerApi for VoidCallback {
    fn as_js_function(&self) -> &Function {
    	let bindgen_closure: &Closure<dyn Fn(JsValue)> = &self.bindgen_closure;
    	bindgen_closure.as_ref().unchecked_ref()
    }
}

