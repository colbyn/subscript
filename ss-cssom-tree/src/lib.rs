//! This will be created and updated internally by the Subscript program.
#![allow(dead_code, unused, unused_variables)]

use std::hash::{Hash, Hasher};
use ss_css_types::internal::*;
use ss_trees::tree::map::{SMap, MapApi};
use ss_web_utils::{dom, js, js::console, dom::DomRef};

pub type HashKey = u64;

#[derive(Debug)]
pub struct GlobalCssMount {
	dom_ref: dom::Tag,
	nodes: SMap<GlobalCssMount, u64, LiveStyling, Stylesheet>,
}


impl PartialEq for GlobalCssMount {
    fn eq(&self, other: &GlobalCssMount) -> bool {
    	self.nodes == other.nodes
    }
}
impl Default for GlobalCssMount {
	fn default() -> Self {
		let window = dom::window();
		let dom_ref = window.document.create_element("style");
		window.document.body.append_child(&dom_ref);
		GlobalCssMount {
			dom_ref,
			nodes: SMap::default(),
		}
	}
}

#[derive(Debug)]
pub struct LiveStyling {
	dom_ref: dom::Text,
	hash: u64,
	stylesheet: Stylesheet
}

impl PartialEq for LiveStyling {
    fn eq(&self, other: &LiveStyling) -> bool {
    	self.hash == other.hash &&
        self.stylesheet == other.stylesheet
    }
}


pub struct CssMapApi {
	window: dom::Window,
}

impl Default for CssMapApi {
	fn default() -> Self {
		CssMapApi {
			window: dom::window(),
		}
	}
}

impl MapApi<GlobalCssMount, u64, LiveStyling, Stylesheet> for CssMapApi {
	fn create(&self, attached: &GlobalCssMount, key: &u64, new: Stylesheet) -> LiveStyling {
		let stringified = new.render_css_syntax();
		let dom_ref = self.window.document.create_text_node(stringified.as_str());
		attached.dom_ref.append_child(&dom_ref);
		LiveStyling {
			dom_ref,
			hash: calculate_hash(&new),
			stylesheet: new,
		}
	}
	fn modified(&self, attached: &GlobalCssMount, key: &u64, old: &mut LiveStyling, new: Stylesheet) {
		let new_stringified = new.render_css_syntax();
		old.dom_ref.set_text_content(&new_stringified);
		old.hash = calculate_hash(&new);
		old.stylesheet = new;
	}
	fn remove(&self, attached: &GlobalCssMount, key: u64, old: LiveStyling) {
		attached.dom_ref.remove_child(&old.dom_ref);
	}
	fn unchanged(&self, old: &LiveStyling, new: &Stylesheet) -> bool {
		&old.stylesheet == new
	}
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
	use std::collections::hash_map::DefaultHasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

