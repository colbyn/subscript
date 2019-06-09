use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::closure::Closure;
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
        let window = Window {
        	document: Document {
        		body: Body {
        			instance: From::from(body_instance),
        		},
			    instance: From::from(document_instance),
        	},
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
}
#[derive(Clone, Debug)]
pub struct Document {
	pub instance: JsValue,
	pub body: Body,
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
        Element {instance: From::from(instance)}
	}
	pub fn create_text_node(&self, value: &str) -> Text {
		let instance = self.instance_as_document().create_text_node(value);
	    Text {instance: From::from(instance)}
	}
}
