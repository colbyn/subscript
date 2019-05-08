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
// CONSOLE
///////////////////////////////////////////////////////////////////////////////
pub mod console {
    use super::*;
    
    pub fn log(value: impl Loggable) {
        match value.to_js_value() {
            Either::Left(x) => {
                web_sys::console::log_1(&x);
            }
            Either::Right(x) => {
                web_sys::console::log_1(x);
            }
        }
    }
    pub fn warn(value: impl Loggable) {
        match value.to_js_value() {
            Either::Left(x) => {
                web_sys::console::warn_1(&x);
            }
            Either::Right(x) => {
                web_sys::console::warn_1(x);
            }
        }
    }
    
    pub trait Loggable {
        fn to_js_value(&self) -> Either<JsValue, &JsValue>;
    }
    impl Loggable for &str {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Left(JsValue::from_str(self))
        }
    }
    impl Loggable for String {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Left(JsValue::from_str(self.as_str()))
        }
    }
    impl Loggable for JsValue {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Right(&self)
        }
    }
    impl Loggable for &JsValue {
        fn to_js_value(&self) -> Either<JsValue, &JsValue> {
            Either::Right(self.clone())
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// BROWSER
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Browser {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub body: DomRef,
}

impl Browser {
    pub fn new() -> Self {
        Browser {
            window: internal::get_window(),
            document: internal::get_document(),
            body: DomRef {
                dom_ref_as_element: {
                    let x: JsValue = From::from(internal::get_body());
                    From::from(x)
                },
                dom_ref_as_node: From::from(internal::get_body()),
            },
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// DOM UTILS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct DomRef {
    pub dom_ref_as_node: web_sys::Node,
    pub dom_ref_as_element: web_sys::Element,
}

impl Hash for DomRef {
    fn hash<H: Hasher>(&self, state: &mut H) {}
}

impl DomRef {
    pub fn new(tag: &str) -> Self {
        let element = internal::new_element(&tag.to_owned());
        DomRef {
            dom_ref_as_node: From::from(element.clone()),
            dom_ref_as_element: element,
        }
    }
    pub fn new_text(value: &str) -> Self {
        let value: web_sys::Text = internal::new_text(&value.to_owned().clone());
        let value: JsValue = From::from(value);
        DomRef {
            dom_ref_as_node: From::from(value.clone()),
            dom_ref_as_element: From::from(value),
        }
    }
    pub fn clone_node(&self, deep: bool) -> DomRef {
        let new_dom_ref = {
            if deep {
                self.dom_ref_as_node
                    .clone_node_with_deep(true)
                    .expect("cloneNode failed")
            } else {
                self.dom_ref_as_node
                    .clone_node()
                    .expect("cloneNode failed")
            }
        };
        DomRef {
            dom_ref_as_element: {
                let x: JsValue = From::from(new_dom_ref.clone());
                From::from(x)
            },
            dom_ref_as_node: new_dom_ref,
        }
    }
    pub fn add_event_listener(&self, event_name: &str, callback: &js_sys::Function) {
        self.dom_ref_as_node.add_event_listener_with_callback(event_name, &callback)
            .expect("addEventListener failed");
    }
    pub fn remove_event_listener(&self, event_name: &str, callback: &js_sys::Function) {
        self.dom_ref_as_node.remove_event_listener_with_callback(event_name, &callback)
            .expect("removeEventListener failed");
    }
    pub fn set_attribute(&self, key: &str, value: &str) {
        self.dom_ref_as_element.set_attribute(key, value)
            .expect("setAttribute failed");
    }
    pub fn remove_attribute(&self, key: &str) {
        self.dom_ref_as_element.remove_attribute(key)
            .expect("removeAttribute failed");
    }
    pub fn set_text_content(&self, value: &str) {
        self.dom_ref_as_node.set_text_content(Some(value));
    }
    pub fn append_child(&self, child: &DomRef) {
        self.dom_ref_as_node
            .append_child(&child.dom_ref_as_node)
            .expect("appendChild failed");
    }
    pub fn remove_child(&self, child: &DomRef) {
        self.dom_ref_as_node
            .remove_child(&child.dom_ref_as_node)
            .expect("removeChild failed");
    }
    pub fn replace_child(&self, new_child: &DomRef, old_child: &DomRef) {
        self.dom_ref_as_node
            .replace_child(&new_child.dom_ref_as_node, &old_child.dom_ref_as_node)
            .expect("replacedNode failed");
    }
}


///////////////////////////////////////////////////////////////////////////////
// JAVASCRIPT CALLBACKS - Immediate
///////////////////////////////////////////////////////////////////////////////


#[derive(Clone)]
pub struct Callback<Return> {
    pub function: Rc<Fn(JsValue) -> Option<Return>>,
    pub function_wrapper: Rc<Closure<dyn Fn(JsValue)>>,
    pub js_function: Rc<js_sys::Function>,
    pub output_queue: Rc<RefCell<VecDeque<Return>>>,
}

impl<Return: Clone + Debug + 'static> Callback<Return> {
    pub fn new(function: Rc<Fn(JsValue) -> Option<Return>>) -> Self {
        use wasm_bindgen::JsCast;
        let queue: Rc<RefCell<VecDeque<Return>>> =
            Rc::new(RefCell::new(VecDeque::new()));
        let function_wrapper: Closure<dyn Fn(JsValue)> = Closure::wrap(Box::new({
            let function = function.clone();
            let queue = queue.clone();
            move |value: JsValue| {
                match function(value) {
                    Some(x) => queue.borrow_mut().push_back(x),
                    None => (),
                }
            }
        }));
        let js_function: &js_sys::Function = function_wrapper.as_ref().unchecked_ref();
        let js_function: js_sys::Function = js_function.clone();
        let callback = Callback {
            function: function,
            function_wrapper: Rc::new(function_wrapper),
            js_function: Rc::new(js_function),
            output_queue: queue,
        };
        callback
    }
    pub fn drain(&self) -> Vec<Return> {
        let xs: Vec<Return> = self.output_queue.borrow_mut().drain(..).collect();
        xs
    }
}
impl<Msg: Clone> Debug for Callback<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "Callback")
    }
}
impl<Msg: Clone> PartialEq for Callback<Msg> {
    fn eq(&self, other: &Callback<Msg>) -> bool {true}
}



///////////////////////////////////////////////////////////////////////////////
// MISC UTILS
///////////////////////////////////////////////////////////////////////////////
pub mod internal {
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
    pub fn new_element(tag: &String) -> web_sys::Element {
        let document = get_document();
        let result = document.create_element(tag.as_str())
            .expect("failed to create element");
        result
    }
    pub fn new_text(value: &String) -> web_sys::Text {
        let document = get_document();
        let result = document.create_text_node(value.as_str());
        result
    }
}


///////////////////////////////////////////////////////////////////////////////
// LOCAL-STORAGE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct LocalStorage {
    object_ref: web_sys::Storage
}

impl LocalStorage {
    pub fn new() -> Self {
        let object_ref = internal::get_window()
            .local_storage()
            .expect("localStorage failed")
            .expect("localStorage missing");
        LocalStorage {
            object_ref: object_ref,
        }
    }
    pub fn get<Value>(&self, key: &str) -> Option<Value>
    where
        Value: DeserializeOwned
    {
        let value = self.object_ref
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
            Ok(value) => self.object_ref
                .set_item(key, value.as_str())
                .expect("setItem method failed")
        }
    }
    pub fn remove(&self, key: &str) {
        self.object_ref
            .remove_item(key)
            .expect("removeItem method failed")
    }
}



