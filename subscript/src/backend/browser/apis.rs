use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::closure::Closure;
use serde::{self, Serialize, Deserialize, de::DeserializeOwned};

use crate::backend::browser::display::*;
use crate::backend::browser::utils::{is_svg_tag};

///////////////////////////////////////////////////////////////////////////////
// WINDOW GETTER
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_WINDOW: Window = {
    	let window_instance: web_sys::Window = web_sys::window().expect("window not available");
    	let document_instance: web_sys::Document = window_instance.document().expect("document not available");
    	let body_instance = document_instance.body().expect("document.body not available");
        let local_storage_instance = window_instance
            .local_storage()
            .expect("localStorage failed")
            .expect("localStorage missing");
        let location_instance = window_instance
            .location();
        let history_instance = window_instance
            .history()
            .expect("window.history getter failed");
        let window = Window {
        	document: Document {
        		body: Body {
        			instance: From::from(body_instance),
        		},
			    instance: From::from(document_instance),
        	},
            local_storage: Storage {
                instance: local_storage_instance,
            },
            location: Location {instance: location_instance},
            history: History {instance: history_instance},
            instance: From::from(window_instance),
        };
        window
    };
}


pub fn window() -> Window {
    let win = GLOBAL_WINDOW.with(|win| win.clone());
    win
}

///////////////////////////////////////////////////////////////////////////////
// DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct Window {
	pub instance: JsValue,
	pub document: Document,
    pub local_storage: Storage,
    pub location: Location,
    pub history: History,
}
#[derive(Clone, Debug)]
pub struct Document {
	pub instance: JsValue,
	pub body: Body,
}
#[derive(Clone, Debug)]
pub struct Storage {
    pub instance: web_sys::Storage
}
#[derive(Clone, Debug)]
pub struct Location {
    pub instance: web_sys::Location
}
#[derive(Clone, Debug)]
pub struct History {
    pub instance: web_sys::History
}


///////////////////////////////////////////////////////////////////////////////
// METHODS
///////////////////////////////////////////////////////////////////////////////

impl Window {
	pub fn instance_as_window(&self) -> web_sys::Window {
		From::from(self.instance.clone())
	}
	pub fn request_animation_frame(&self, callback: impl FnOnce() + 'static) -> Closure<FnMut()> {
	    let callback: Closure<FnMut()> = Closure::once(callback);
	    let js_function: &js_sys::Function = callback.as_ref().unchecked_ref();
	    self.instance_as_window()
	    	.request_animation_frame(js_function)
	        .expect("request_animation_frame failed");
	    callback
	}
	pub fn set_timeout(&self, timeout: i32, callback: impl FnOnce() + 'static) -> Closure<FnMut()> {
	    let callback: Closure<FnMut()> = Closure::once(callback);
	    let js_function: &js_sys::Function = callback.as_ref().unchecked_ref();
	    self.instance_as_window().set_timeout_with_callback_and_timeout_and_arguments_0(
	        js_function,
	        timeout
	    ).expect("set_timeout_with_callback_and_timeout_and_arguments_0 failed");
	    callback
	}
}

impl Document {
	pub fn instance_as_document(&self) -> web_sys::Document {
		From::from(self.instance.clone())
	}
	pub fn create_element(&self, tag: &str) -> Element {
        let instance = if is_svg_tag(tag) {
        	self.instance_as_document()
        		.create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
        	    .expect("failed to create svg-element")
        } else {
            self.instance_as_document()
            	.create_element(tag)
                .expect("failed to create element")
        };
        let class_list = ClassList {
        	instance: From::from(instance.class_list())
        };
        Element {instance: From::from(instance), class_list}
	}
	pub fn create_text_node(&self, value: &str) -> Text {
		let instance = self.instance_as_document().create_text_node(value);
	    Text {instance: From::from(instance)}
	}
}

impl Storage {
    pub fn get<Value>(&self, key: &str) -> Option<Value> where Value: DeserializeOwned {
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
impl History {
    pub fn push_state(&self, url_path: &str) {
        self.instance.push_state_with_url(
            &JsValue::null(),
            "",
            Some(url_path)
        )
        .expect("pushState failed");
    }
}
impl Location {
    pub fn pathname(&self) -> String {
        self.instance
            .pathname()
            .expect("pathname failed")
    }
}

