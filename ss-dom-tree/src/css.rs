use std::iter::FromIterator;
use std::rc::*;
use std::cell::*;
use std::collections::*;
use wasm_bindgen::JsValue;
use ss_view_tree::styling::*;
use ss_view_tree::styling::syntax::*;
use ss_web_utils::dom::DomRef;
use ss_web_utils::{dom, js, js::console, prelude::*};
use ss_trees::ext::map::{SMap, MapApi};
use crate::LiveStylesheet;

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type MediaSelectorType = String;

///////////////////////////////////////////////////////////////////////////////
// CRATE-LEVEL EXTERNAL API
///////////////////////////////////////////////////////////////////////////////

pub(crate) fn upsert(sheet: &LiveStylesheet) {
	GLOBAL_CSS_REGISTRY.with(move |reg| {
		reg.borrow_mut().upsert(sheet);
	});
}

///////////////////////////////////////////////////////////////////////////////
// GLOBAL-CSS-REGISTRY
///////////////////////////////////////////////////////////////////////////////

thread_local! {
    pub static GLOBAL_CSS_REGISTRY: RefCell<GlobalCssRegistry> = {
    	RefCell::new(GlobalCssRegistry::Pending)
    };
}

#[derive(Debug, PartialEq)]
pub enum GlobalCssRegistry {
	Pending,
	Live(LiveCssRegistry)
}

impl GlobalCssRegistry {
	pub fn get_live(&mut self) -> &mut LiveCssRegistry {
		let mut start = || -> Self {
			GlobalCssRegistry::Live(LiveCssRegistry {
				mount: {
					let window = dom::window();
					let wrapper = dom::Tag::new("div");
					let local = dom::Tag::new("style");
					let state = dom::Tag::new("style");
					let media = dom::Tag::new("style");
					wrapper.set_attribute("subscript-styling-wrapper", "");
					wrapper.set_attribute("style", "display: none;");
					wrapper.append_child(&local);
					wrapper.append_child(&state);
					wrapper.append_child(&media);
					// INSERT INTO DOM TREE - BODY CHILD - FIRST ELEMENT
					let wrapper_ref: JsValue = wrapper.dom_ref().clone();
					let wrapper_ref: web_sys::Element = From::from(wrapper_ref);
					let body: dom::Body = dom::window().document.body;
					let body: wasm_bindgen::JsValue = body.dom_ref().clone();
					let body: web_sys::Element = web_sys::Element::from(body);
					body.insert_adjacent_element("afterbegin", &wrapper.dom_ref_as_element);
					GlobalCssMount {wrapper,local,state,media}
				},
				added: HashSet::new(),
			})
		};
		match self {
			GlobalCssRegistry::Pending => {
				*self = start();
				match self {
					GlobalCssRegistry::Live(x) => {x}
					GlobalCssRegistry::Pending => {panic!()}
				}
			}
			GlobalCssRegistry::Live(x) => x
		}
	}
	pub fn upsert(&mut self, sheet: &LiveStylesheet) {
		let live = self.get_live();
		live.upsert(sheet);
	}
}


#[derive(Debug, PartialEq)]
pub struct LiveCssRegistry {
	mount: GlobalCssMount,
	added: HashSet<u64>,
}

impl LiveCssRegistry {
	pub fn upsert(&mut self, sheet: &LiveStylesheet) {
		if !self.added.contains(&sheet.css_id.borrow()) {
			// SETUP
			let css_id = sheet.css_id.borrow().clone();
			self.added.insert(css_id.clone());
			let RenderedStylesheet{local, state, media, keyframes} = sheet.value
				.borrow()
				.render_css_syntax(crate::trim_css_id(&css_id).as_str());
			// STYLESHEET LOCAL
			{
				let local = dom::window().document.create_text_node(&local.0);
				self.mount.local.append_child(&local);
			}
			// STYLESHEET STATE
			for entry in state {
				let text = dom::window().document.create_text_node(&(entry.1).0);
				self.mount.state.append_child(&text);
			}
			// STYLESHEET KEYFRAMES
			for entry in keyframes {
				let text = dom::window().document.create_text_node(&(entry.1).0);
				self.mount.state.append_child(&text);
			}
			// STYLESHEET MEDIA
			for entry in media {
				let text = format!(
					"@media {key} {{{inner}}}",
					key=entry.0,
					inner=(entry.1).0,
				);
				let text = dom::window().document.create_text_node(text.as_str());
				self.mount.media.append_child(&text);
			}
		}
	}
}





///////////////////////////////////////////////////////////////////////////////
// GLOBAL-STYLESHEET-CONTROLLER - DATA
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct GlobalCssMount {
	wrapper: dom::Tag,
	local: dom::Tag,
	state: dom::Tag,
	media: dom::Tag,
}



